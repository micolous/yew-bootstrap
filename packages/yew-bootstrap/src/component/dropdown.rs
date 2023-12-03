//! Dropdown menu
//!
//! **Work in progress**: this currently conflicts with Bootstrap's JavaScript.
//!
//! TODO: implement children as ChildrenWithProperties
use popper_rs::{
    modifier::{Modifier, Offset},
    options::Options,
    sys::types::{Placement as PopperPlacement, Strategy},
    yew::use_popper,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement, Node};
use yew::{platform::spawn_local, prelude::*};

#[derive(Debug, PartialEq, Eq)]
pub enum DropdownCloseRequest {
    EscapeKey,
    FocusLoss,
}

#[derive(Properties, Clone, PartialEq)]
pub struct DropdownMenuProps {
    /// The node which this menu is attached to.
    pub target: NodeRef,

    /// Content of the menu.
    #[prop_or_default]
    pub children: Children,

    /// Placement strategy of the menu.
    #[prop_or(PopperPlacement::BottomStart)]
    pub placement: PopperPlacement,

    /// When `true`, show the menu.
    #[prop_or_default]
    pub show: bool,

    /// Callback fired whenever it looks like the user is trying to explicitly
    /// dismiss the dropdown menu, by pressing the `escape` key or it losing
    /// focus.
    ///
    /// If this callback is not provided, the only way to dismiss the menu is by
    /// some other event handler setting [`show`][Self::show] to `false`.
    #[prop_or_default]
    pub on_close_requested: Option<Callback<DropdownCloseRequest>>,
}

#[function_component]
pub fn DropdownMenu(props: &DropdownMenuProps) -> Html {
    let dropdown_ref = use_node_ref();
    // Set to `true` after Popper has finished positioning the drop-down.
    let shown = use_state_eq(|| false);
    let options = use_memo(props.placement, |placement| Options {
        placement: (*placement).into(),
        modifiers: vec![Modifier::Offset(Offset {
            skidding: 0,
            distance: 2,
        })],
        strategy: Strategy::Absolute,
        ..Default::default()
    });

    let popper = use_popper(props.target.clone(), dropdown_ref.clone(), options).unwrap();

    let mut class = classes!["dropdown-menu"];
    if props.show {
        class.push("show");
    }
    let data_show = props.show.then(AttrValue::default);

    use_effect_with(
        (shown.clone(), dropdown_ref.clone()),
        |(shown, dropdown_ref)| {
            if !**shown {
                return;
            }

            // Focus the first item in the drop-down when we are shown, and
            // after Popper has finished.
            let Some(dropdown_elem) = dropdown_ref.cast::<HtmlElement>() else {
                return;
            };
            // TODO: replace this when there's structured child nodes
            let Ok(focusables) = dropdown_elem
                .query_selector_all(":scope .dropdown-item:not(.disabled):not(:disabled)")
            else {
                return;
            };
            if focusables.length() == 0 {
                return;
            }
            let Some(f) = focusables.get(0) else {
                return;
            };
            let Some(s) = f.dyn_ref::<HtmlElement>() else {
                return;
            };

            let _ = s.focus();
        },
    );

    let onfocusout = {
        let dropdown_ref = dropdown_ref.clone();
        let on_close_requested = props.on_close_requested.clone();
        Callback::from(move |evt: FocusEvent| {
            let Some(dropdown_elem) = dropdown_ref.get() else {
                return;
            };
            let Some(on_close_requested) = &on_close_requested else {
                return;
            };

            if let Some(target) = evt.related_target() {
                if let Ok(target) = target.dyn_into::<Node>() {
                    // Check if the new thing being focussed is us or our child
                    if target != dropdown_elem && !dropdown_elem.contains(Some(&target)) {
                        on_close_requested.emit(DropdownCloseRequest::FocusLoss);
                    }
                };
            } else {
                // No related target
                on_close_requested.emit(DropdownCloseRequest::FocusLoss);
            }
        })
    };

    let onkeydown = {
        let dropdown_ref = dropdown_ref.clone();
        let on_close_requested = props.on_close_requested.clone();
        let shown = shown.clone();
        Callback::from(move |evt: KeyboardEvent| {
            if !*shown {
                return;
            }
            let Some(target) = evt.target() else {
                return;
            };
            let Ok(target) = target.dyn_into::<HtmlElement>() else {
                return;
            };

            let key = evt.key();
            let is_escape = key.eq_ignore_ascii_case("Escape");
            let is_arrow_up = key.eq_ignore_ascii_case("ArrowUp");
            let is_arrow_down = key.eq_ignore_ascii_case("ArrowDown");
            let is_enter = key.eq_ignore_ascii_case("Enter");
            if target.dyn_ref::<HtmlInputElement>().is_some()
                || target.dyn_ref::<HtmlTextAreaElement>().is_some()
            {
                if !is_escape {
                    return;
                }
            } else {
                if !(is_escape || is_arrow_down || is_arrow_up || is_enter) {
                    return;
                }
            }

            evt.prevent_default();
            if is_escape {
                if let Some(on_close_requested) = &on_close_requested {
                    on_close_requested.emit(DropdownCloseRequest::EscapeKey);
                }
                return;
            }

            evt.stop_propagation();
            if is_enter {
                // Make a click event for the selected element
                target.click();
                return;
            }

            // Make a list of all the focusable elements currently in the
            // drop-down.
            let Some(dropdown_elem) = dropdown_ref.cast::<HtmlElement>() else {
                return;
            };

            // TODO: replace this when there's structured child nodes
            let focusables = dropdown_elem
                .query_selector_all(":scope .dropdown-item:not(.disabled):not(:disabled)")
                .unwrap();
            if focusables.length() == 0 {
                return;
            }

            let mut current_pos = 0;
            for i in 0..focusables.length() {
                let Some(f) = focusables.item(i) else {
                    break;
                };

                let Some(s) = f.dyn_ref::<HtmlElement>() else {
                    continue;
                };

                if &target == s {
                    current_pos = i;
                    break;
                }
            }

            let i = if is_arrow_up {
                // Find previous focusable
                if current_pos == 0 {
                    focusables.length() - 1
                } else {
                    current_pos - 1
                }
            } else {
                // arrow_down
                if current_pos >= (focusables.length() - 1) {
                    0
                } else {
                    current_pos + 1
                }
            };

            let Some(f) = focusables.item(i) else {
                return;
            };

            let Some(s) = f.dyn_ref::<HtmlElement>() else {
                return;
            };

            s.focus().unwrap();
        })
    };

    use_effect_with(
        (props.show, popper.instance.clone()),
        move |(show, popper)| {
            if *show {
                let popper = popper.clone();
                let shown = shown.clone();

                spawn_local(async move {
                    popper.update().await;
                    shown.set(true);
                });
            } else {
                shown.set(false);
            }
        },
    );

    html! {
        <ul
            {class}
            data-show={data_show}
            ref={&dropdown_ref}
            style={&popper.state.styles.popper}
            {onfocusout}
            {onkeydown}
        >
            { for props.children.iter() }
        </ul>
    }
}

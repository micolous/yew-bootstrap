use popper_rs::{
    modifier::{Modifier, Offset},
    options::Options,
    sys::types::{Placement as PopperPlacement, Strategy},
    yew::use_popper,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlElement, Node, HtmlInputElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, prelude::*};

#[derive(Debug, PartialEq, Eq)]
pub enum DropdownCloseRequest {
    Click,
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

    #[prop_or_default]
    pub show: bool,

    /// Callback fired whenever it looks like the user is trying to explicitly
    /// dismiss the dropdown menu, by clicking outside of it, or pressing the
    /// `escape` key.
    ///
    /// If this callback is not provided, then the only way to dismiss the menu
    /// is by some other event handler setting [`show`][Self::show] to `false`.
    #[prop_or_default]
    pub on_close_requested: Option<Callback<DropdownCloseRequest>>,
}

#[function_component]
pub fn DropdownMenu(props: &DropdownMenuProps) -> Html {
    let dropdown_ref = use_node_ref();
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
    let popper_style = popper.state.styles.popper.clone();
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
            let Some(dropdown_elem) = dropdown_ref.cast::<HtmlElement>() else {
                return;
            };

            let _ = dropdown_elem.focus();
        },
    );

    // TODO: implement keyboard events
    // TODO: implement click-out event

    let on_close_request = {
        let cb = props.on_close_requested.clone();
        Callback::from(move |evt: DropdownCloseRequest| {
            if let Some(cb) = &cb {
                cb.emit(evt);
            }
        })
    };

    // Register global event handlers
    // use_effect_with(
    //     (props.target.clone(), dropdown_ref.clone(), shown.clone()),
    //     |(target, dropdown_ref, shown)| {
    //         let document = gloo::utils::document_element();
    //         // let dropdown_ref = dropdown_ref.clone();
    //         let shown = shown.clone();
    //         let close_request = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| {
    //             if e.default_prevented() || !*shown {
    //                 return;
    //             }
    //             // let Some(dropdown_elem) = dropdown_ref.cast::<HtmlElement>() else {
    //             //     return;
    //             // };
    //             // let classes = Classes::from(dropdown_elem.class_name());
    //             // if !classes.contains("show") {
    //             //     return;
    //             // }

    //             // if let Some(event_target_elem) = e.target_dyn_into::<HtmlElement>() {
    //             //     if target_elem == event_target_elem {
    //             //         // Ignore clicking on the
    //             //         return;
    //             //     }
    //             // }
    //             // on_close_request.emit(DropdownCloseRequest::Click);
    //         }));

    //         let _ = document
    //             .add_event_listener_with_callback("click", close_request.as_ref().unchecked_ref());

    //         move || {
    //             let _ = document.remove_event_listener_with_callback(
    //                 "click",
    //                 close_request.as_ref().unchecked_ref(),
    //             );
    //             drop(close_request);
    //         }
    //     },
    // );

    

    let onfocusout = {
        let dropdown_ref = dropdown_ref.clone();
        let on_close_request = on_close_request.clone();
        Callback::from(move |evt: FocusEvent| {
            let Some(dropdown_elem) = dropdown_ref.get() else {
                return;
            };

            if let Some(target) = evt.related_target() {
                if let Ok(target) = target.dyn_into::<Node>() {
                    // Check if the new thing being focussed is us or our child
                    if target != dropdown_elem && !dropdown_elem.contains(Some(&target)) {
                        on_close_request.emit(DropdownCloseRequest::FocusLoss);
                    }
                };
            } else {
                // No related target
                on_close_request.emit(DropdownCloseRequest::FocusLoss);
            }

        })
    };

    let onkeydown = {
        let dropdown_ref = dropdown_ref.clone();
        let on_close_request = on_close_request.clone();
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
            if target.dyn_ref::<HtmlInputElement>().is_some() || target.dyn_ref::<HtmlTextAreaElement>().is_some() {
                if !is_escape {
                    return;
                }
            } else {
                if !(is_escape || is_arrow_down || is_arrow_up) {
                    return;
                }
            }

            evt.prevent_default();
            if is_escape {
                on_close_request.emit(DropdownCloseRequest::EscapeKey);
                return;
            }

            // Make a list of all the focusabl elements currently in the
            // drop-down.
            let Some(dropdown_elem) = dropdown_ref.cast::<HtmlElement>() else {
                return;
            };

            let focusables = dropdown_elem.query_selector_all(":scope .dropdown-item:not(.disabled):not(:disabled)").unwrap();
            if focusables.length() == 0 {
                panic!("no focusables");
                return;
            }

            let mut current_pos = 0;
            for i in 0..focusables.length() {
                let Some(f) = focusables.item(i) else {
                    break;
                };

                let Some(s) = f.dyn_ref::<HtmlElement>() else {
                    panic!("not html element? {i}");
                };

                if &target == s {
                    current_pos = i;
                    break;
                }
            }

            

            let i = if is_arrow_up {
                // Find previous focusable
                if current_pos == 0 { focusables.length() - 1 } else { current_pos - 1 }
                
            } else { // arrow_down
                if current_pos >= (focusables.length() - 1) { 0 } else { current_pos + 1 }  
            };

            let Some(f) = focusables.item(i) else {
                panic!("not node {i}");
            };

            let Some(s) = f.dyn_ref::<HtmlElement>() else {
                panic!("not html element? {i}");
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
            style={&popper_style}
            tabindex="0"
            {onfocusout}
            {onkeydown}
        >
            { for props.children.iter() }
        </ul>
    }
}

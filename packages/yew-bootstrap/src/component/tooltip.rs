//! Implements tooltip suppport.
//!
//! `yew` presues it has exclusive control of the DOM, which conflicts with the
//! Bootstrap's assumption that it also has exclusive control of the DOM.
//!
//! So, we need to re-implement the Tooltip plugin using `yew`...
//!
//! * <https://github.com/react-bootstrap/react-bootstrap/blob/master/src/Tooltip.tsx>
//! * <https://github.com/twbs/bootstrap/blob/main/js/src/tooltip.js>

use popper_rs::{
    prelude::{use_popper, Modifier, Offset, Options, Placement as PopperPlacement, Strategy},
    state::ApplyAttributes,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{console, HtmlElement};
use yew::{platform::spawn_local, prelude::*};

// use crate::util::{PopperConfig, Popper};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Placement {
    #[default]
    Auto,
    Top,
    Bottom,
    Left,
    Right,
}

impl From<Placement> for PopperPlacement {
    fn from(value: Placement) -> Self {
        match value {
            Placement::Auto => PopperPlacement::Auto,
            Placement::Bottom => PopperPlacement::Bottom,
            Placement::Left => PopperPlacement::Left,
            Placement::Right => PopperPlacement::Right,
            Placement::Top => PopperPlacement::Top,
        }
    }
}

impl Placement {
    const fn bootstrap_class(&self) -> &'static str {
        match self {
            Placement::Auto => "bs-tooltip-auto",
            Placement::Left => "bs-tooltip-left",
            Placement::Top => "bs-tooltip-top",
            Placement::Right => "bs-tooltip-right",
            Placement::Bottom => "bs-tooltip-bottom",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TooltipProps {
    /// The node which this tooltip is attached to.
    ///
    /// If the `target` can be `disabled`, pass the same value to
    /// [Tooltip's `disabled` property][Self::disabled] to ensure that it will
    /// be automatically hidden even if it had focus on was being hovered.
    ///
    /// If the `target` has an `id` set, the [Tooltip] will set its
    /// `aria-describedby` attribute whenever it is visible.
    pub target: NodeRef,

    /// Content of the tooltip.
    #[prop_or_default]
    pub children: Children,

    /// Placement strategy of the tooltip.
    #[prop_or_default]
    pub placement: Placement,

    /// Use fade transition when showing or hiding the tooltip.
    #[prop_or_default]
    pub fade: bool,

    /// If `true`, always show the tooltip, regardless of focus state.
    ///
    /// [`disabled = true`][TooltipProps::disabled] overrides this option.
    #[prop_or_default]
    pub show: bool,

    /// Show the tooltip when the [`target`][Self::target] node recieves input
    /// or keyboard focus.
    ///
    /// This defaults to `true`, but [will not trigger on `disabled` elements][0].
    ///
    /// If the [`target`][Self::target] element can be disabled, pass the same
    /// state to this component's [`disabled` property][Self::disabled] to
    /// ensure that the [Tooltip] will be automatically hidden.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or(true)]
    pub trigger_on_focus: bool,

    /// Show the tooltip when the [`target`][Self::target] node has the mouse
    /// cursor hovered over it.
    ///
    /// This defaults to `true`, but [will not trigger on `disabled` elements][0].
    ///
    /// **Note:** this option has no effect on touchscreen devices. Make sure
    /// there are other ways of displaying the tooltip.
    ///
    /// If the [`target`][Self::target] element can be disabled, pass the same
    /// state to this component's [`disabled` property][Self::disabled] to
    /// ensure that the [Tooltip] will be automatically hidden.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or(true)]
    pub trigger_on_hover: bool,

    /// If `true`, always hide the tooltip. *This overrides all other
    /// conditions.*
    ///
    /// The tooltip will remain part of the DOM.
    ///
    /// [Disabled elements don't fire events][0], including `focusout` on a
    /// currently-focused element and `mouseleave` of a currently-hovered
    /// element.
    ///
    /// This property allows you to hide a [Tooltip] which has
    /// [`trigger_on_focus = true`][Self::trigger_on_focus] or
    /// [`trigger_on_hover = true`][Self::trigger_on_hover] whenever the
    /// [`target`][Self::target] is disabled.
    ///
    /// **Warning:** entirely removing the [Tooltip] from the DOM whenever the
    /// [`target`][Self::target] is disabled can cause strange behaviour if the
    /// that tooltip is currently being displayed.
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/tooltips/#disabled-elements
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let tooltip_ref = use_node_ref();
    let target_id = props.target.cast::<HtmlElement>().and_then(|e| {
        let id = e.id();
        (!id.is_empty()).then_some(id)
    });
    let tooltip_id = target_id.as_ref().map(|id| format!("ybst-{id}"));

    // Adapted from https://github.com/ctron/popper-rs/blob/main/examples/yew/src/example/basic.rs
    let options = use_memo(props.placement, |placement| Options {
        placement: (*placement).into(),
        modifiers: vec![Modifier::Offset(Offset {
            skidding: 0,
            distance: 8,
        })],
        strategy: Strategy::Absolute,
        ..Default::default()
    });

    let popper = use_popper(props.target.clone(), tooltip_ref.clone(), options).unwrap();
    {
        let popper = popper.instance.clone();
        use_effect(|| {
            spawn_local(async move {
                popper.update().await;
            });
        });
    }

    let focused = use_state_eq(|| false);
    let hovered = use_state_eq(|| false);

    let onshow = {
        let focused = focused.clone();
        let hovered = hovered.clone();
        let popper = popper.instance.clone();
        Callback::from(move |evt_type: String| {
            match evt_type.as_str() {
                "mouseenter" => hovered.set(true),
                "focusin" => focused.set(true),
                _ => {
                    return;
                }
            }
            let popper = popper.clone();

            spawn_local(async move {
                popper.update().await;
            });
        })
    };

    let onhide = {
        let focused = focused.clone();
        let hovered = hovered.clone();
        Callback::from(move |evt_type: String| match evt_type.as_str() {
            "mouseleave" => hovered.set(false),
            "focusout" => focused.set(false),
            _ => {
                return;
            }
        })
    };

    if props.disabled {
        // Whenever this component is disabled, explicitly set our focus and
        // hover state to false.
        focused.set(false);
        hovered.set(false);
    }
    let show = !props.disabled
        && (props.show
            || (*focused && props.trigger_on_focus)
            || (*hovered && props.trigger_on_hover));
    let data_show = show.then(AttrValue::default);

    use_effect_with(
        (tooltip_ref.clone(), popper.state.attributes.popper.clone()),
        |(tooltip_ref, attributes)| {
            tooltip_ref.apply_attributes(attributes);
        },
    );

    // Attach event handlers. These are always wired up, just we ignore the
    // result when they're disabled.
    use_effect_with(props.target.clone(), |target_ref| {
        let show_listener = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| {
            // console::log_1(&format!("mouse enter event").into());
            onshow.emit(e.type_());
        }));
        let hide_listener = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| {
            // console::log_1(&format!("mouse leave event").into());
            onhide.emit(e.type_());
        }));
        let parent_elem = target_ref.cast::<HtmlElement>();

        if let Some(parent_elem) = &parent_elem {
            let _ = parent_elem.add_event_listener_with_callback(
                "focusin",
                show_listener.as_ref().unchecked_ref(),
            );
            let _ = parent_elem.add_event_listener_with_callback(
                "focusout",
                hide_listener.as_ref().unchecked_ref(),
            );

            let _ = parent_elem.add_event_listener_with_callback(
                "mouseenter",
                show_listener.as_ref().unchecked_ref(),
            );
            let _ = parent_elem.add_event_listener_with_callback(
                "mouseleave",
                hide_listener.as_ref().unchecked_ref(),
            );
        };

        move || {
            if let Some(parent_elem) = parent_elem {
                let _ = parent_elem.remove_event_listener_with_callback(
                    "focusin",
                    show_listener.as_ref().unchecked_ref(),
                );
                let _ = parent_elem.remove_event_listener_with_callback(
                    "focusout",
                    hide_listener.as_ref().unchecked_ref(),
                );
                let _ = parent_elem.remove_event_listener_with_callback(
                    "mouseenter",
                    show_listener.as_ref().unchecked_ref(),
                );
                let _ = parent_elem.remove_event_listener_with_callback(
                    "mouseleave",
                    hide_listener.as_ref().unchecked_ref(),
                );
            }
            drop(show_listener);
            drop(hide_listener);
        }
    });

    use_effect_with(
        (props.target.clone(), tooltip_id, show),
        |(target_ref, tooltip_id, show)| {
            let Some(parent_elem) = target_ref.cast::<HtmlElement>() else {
                return;
            };

            match (tooltip_id, show) {
                (Some(tooltip_id), true) => {
                    let _ = parent_elem.set_attribute("aria-describedby", tooltip_id);
                }
                _ => {
                    let _ = parent_elem.remove_attribute("aria-describedby");
                }
            }
        },
    );

    let mut class = classes!["tooltip"];
    if props.fade {
        class.push("fade");
    }
    let mut popper_style = popper.state.styles.popper.clone();
    if show {
        class.push("show");
        popper_style.remove("z-index");
    } else {
        popper_style.insert("z-index".to_string(), "-100".to_string());
    }
    class.push(props.placement.bootstrap_class());

    html_nested! {
        <div
            ref={&tooltip_ref}
            role="tooltip"
            {class}
            style={&popper_style}
            data-show={&data_show}
        >
            <div
                class="tooltip-arrow"
                data-popper-arrow="true"
                style={&popper.state.styles.arrow}
            />
            <div class="tooltip-inner">
                { for props.children.iter() }
            </div>
        </div>
    }
}

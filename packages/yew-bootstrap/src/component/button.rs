use crate::util::Color; //, Tooltip, TooltipOptions};
// use wasm_bindgen::{JsCast, JsValue};
// use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum ButtonSize {
    Large,
    Normal,
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Normal
    }
}

/// # Button component
/// Button with various properties, including support for opening or closing a modal
/// dialog [crate::component::Modal].
///
/// Buttons can be grouped in a [crate::component::ButtonGroup].
///
/// See [ButtonProps] for a listing of properties.
///
/// ## Example
/// Example of a simple button:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Button;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Button style={Color::Primary} text={ "Button text" }/>
///     }
/// }
/// ```
///
/// A button can be linked to a [crate::component::Modal] dialog or
/// close this modal.
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Button;
/// use yew_bootstrap::component::Modal;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html ! {
///         <>
///             <Modal id="ExampleModal">
///                <Button modal_dismiss={true}>{ "Close the modal" }</Button>
///             </Modal>
///             <Button style={Color::Primary} modal_target={ "ExampleModal" }>
///                 { "Open Modal" }
///             </Button>
///         </>
///     }
/// }
/// ```
///
/// A button may also link to a web page.
///
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Button;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Button style={Color::Primary} text={ "Button text" } url={ "https://getbootstrap.com/docs/5.3/components/buttons/#button-tags" } target={"_blank"} />
///     }
/// }
/// ```
pub struct Button {
    // node_ref: NodeRef,
    // tooltip: Option<Tooltip>,
}

/// # Properties for [Button]
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Optional children
    #[prop_or_default]
    pub children: Children,

    /// Treat button as block that spans the full width of the parent
    #[prop_or_default]
    pub block: bool,

    /// Status of the button. Disabled buttons cannot be clicked.
    #[prop_or_default]
    pub disabled: bool,

    /// Name of the component
    #[prop_or_default]
    pub name: String,

    /// Event called when the button is clicked
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Show button as outlined instead of filled
    #[prop_or_default]
    pub outline: bool,

    /// Size of the button
    #[prop_or_default]
    pub size: ButtonSize,

    /// Color of the button, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Text displayed in the button
    #[prop_or_default]
    pub text: String,

    /// if provided, we will set data-bs-toggle and data-bs-target for modal opening
    #[prop_or_default]
    pub modal_target: Option<String>,

    /// true if this button dismisses the modal that contains it
    #[prop_or_default]
    pub modal_dismiss: bool,

    /// URL to direct to when the button is clicked. This turns the button into
    /// an `<a>` element.
    ///
    /// This property is ignored if the button is `disabled` to
    /// [avoid link functionality caveats][0], which may result in
    /// [slightly different rendering on some browsers / platforms][1].
    ///
    /// [0]: https://getbootstrap.com/docs/5.3/components/buttons/#link-functionality-caveat
    /// [1]: https://getbootstrap.com/docs/5.3/components/buttons/#button-tags
    #[prop_or_default]
    pub url: Option<AttrValue>,

    /// Target frame or window ID for the link. Only used if `url` is set and
    /// the button is not `disabled`.
    #[prop_or_default]
    pub target: Option<AttrValue>,

    #[prop_or_default]
    pub dropdown: bool,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // node_ref: NodeRef::default(),
            // tooltip: None,
        }
    }

    // fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    //     if first_render {
    //         // https://getbootstrap.com/docs/5.3/components/tooltips/#usage
    //         let Some(elem) = self.node_ref.cast::<HtmlElement>() else {
    //             return;
    //         };
    //         self.tooltip = Some(Tooltip::new(elem));
    //     }
    // }

    // fn destroy(&mut self, ctx: &Context<Self>) {
    //     if let Some(tooltip) = self.tooltip.take() {
    //         tooltip.dispose();
    //     }
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("btn");
        if props.outline {
            classes.push(format!("btn-outline-{}", props.style));
        } else {
            classes.push(format!("btn-{}", props.style));
        }
        match props.size {
            ButtonSize::Large => classes.push("btn-lg"),
            ButtonSize::Small => classes.push("btn-sm"),
            _ => (),
        }
        if props.block {
            classes.push("btn-block");
        }
        classes.push(props.class.clone());

        let modal_dismiss = match props.modal_dismiss {
            true => "modal",
            false => "",
        };

        // if let Some(tooltip) = &self.tooltip {
        //     let title = props.title.clone().unwrap_or_default().to_string();
        //     let options = TooltipOptions::new();
        //     options.set_title(title.into());

        //     tooltip.set_content(options);
        //     // todo: tooltip.setContent({ '.title': 'another title' })
        //     // tooltip.set_content();
        // }

        if let Some(target) = &props.modal_target {
            html! {
                <button
                    class={classes}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    onclick={props.onclick.clone()}
                    data-bs-toggle="modal"
                    data-bs-target={format!("#{}",target.clone())}
                    // title={props.title.clone()}
                    ref={props.node_ref.clone()}
                >
                    { &props.text }
                    { for props.children.iter() }
                </button>
            }
        } else if let Some(url) = props.url.as_ref().filter(|_| !props.disabled) {
            html! {
                <a
                    class={classes}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    onclick={props.onclick.clone()}
                    data-bs-dismiss={modal_dismiss}
                    href={url.clone()}
                    target={props.target.clone()}
                    // title={props.title.clone()}
                    ref={props.node_ref.clone()}
                >
                    { &props.text }
                    { for props.children.iter() }
                </a>
            }
        } else if props.dropdown {
            classes.push("dropdown-toggle");
            html! {
                <button
                    class={classes}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    onclick={props.onclick.clone()}
                    aria-expanded="false"
                    data-bs-toggle="dropdown"
                    data-bs-dismiss={modal_dismiss}
                    // title={props.title.clone()}
                    ref={props.node_ref.clone()}
                >
                    { &props.text }
                    { for props.children.iter() }
                </button>
            }
        } else {
            html! {
                <button
                    class={classes}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    onclick={props.onclick.clone()}
                    data-bs-dismiss={modal_dismiss}
                    // title={props.title.clone()}
                    ref={props.node_ref.clone()}
                >
                    { &props.text }
                    { for props.children.iter() }
                </button>
            }
        }
    }
}

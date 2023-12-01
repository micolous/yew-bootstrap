use popper_rs::{
    modifier::{Modifier, Offset},
    options::Options,
    sys::types::{Placement as PopperPlacement, Strategy},
    yew::use_popper,
};
use yew::{platform::spawn_local, prelude::*};

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
}

#[function_component]
pub fn DropdownMenu(props: &DropdownMenuProps) -> Html {
    let dropdown_ref = use_node_ref();
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
    {
        let popper = popper.instance.clone();
        use_effect(|| {
            spawn_local(async move {
                popper.update().await;
            });
        });
    }
    let mut class = classes!["dropdown-menu"];
    let mut popper_style = popper.state.styles.popper.clone();
    if props.show {
        class.push("show");
        popper_style.remove("z-index");
    } else {
        popper_style.insert("z-index".to_string(), "-100".to_string());
    }

    // TODO: implement keyboard events
    // TODO: implement click-out event

    html! {
        <ul
            {class}
            data-show={props.show.then(AttrValue::default)}
            ref={&dropdown_ref}
            style={&popper_style}
        >
            { for props.children.iter() }
        </ul>
    }
}

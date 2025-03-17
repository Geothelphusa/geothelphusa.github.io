use yew::prelude::*;
use crate::styles::*;

pub const LOGO_PATH: &str = "https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg";
#[derive(Properties, PartialEq)]
pub struct MenuButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub is_opened: bool,
}


#[function_component(MenuButton)]
pub fn menu_button(props: &MenuButtonProps) -> Html {

    let class = if props.is_opened {
        classes!(menu_button_style(), "is-opened")
    } else {
        classes!(menu_button_style())
    };

    html! {
        <button id="menuButton" type="button" class={classes!(class, menu_button_style())} aria-labelledby="menuButtonLabel" onclick={props.onclick.clone()}>
            <span class="menu-button__line">
                <span id="menuButtonLabel" style="display: none">{"メニューボタン"}</span>
            </span>
        </button>
    }
}
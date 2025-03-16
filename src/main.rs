mod styles;

use stylist::{yew::styled_component, style};
use yew::prelude::*;
use crate::styles::*;

#[styled_component(App)]

fn app() -> Html {
const LOGO_PATH: &str = "https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg";
    let stylesheet = style!(
        r#"
        .container {
            width: 95%; /* 画面幅に合わせてコンテナの幅を調整 */
            max-width: 1200px; /* 最大幅を設定 */
            margin: 0 auto; /* 中央寄せ */
            height: auto; /* 高さを自動調整 */
        }

        @media (min-width: 768px) {
            .container {
                width: 70%;
            }
        }

        @media (min-width: 1200px) {
            .container {
                width: 50%;
            }
        }
        "#
    )
    .unwrap();

    let logo_path = LOGO_PATH;

    // モードの状態を保持する変数(初期値はライトモード)
    let dark_mode = use_state(|| true);

    let mut main_classes = Classes::new();
    main_classes.push(container_styles());
    if *dark_mode {
        main_classes.push(dark_mode_styles());
    } else {
        main_classes.push(light_mode_styles());
    };

    html! {
            <body class={classes!(base_styles())}>
                <div class={stylesheet}>
                    <nav class={classes!(nav_styles())}>
                        <ul class={css!("display: flex; flex-direction: column; @media (min-width: 768px) {flex-direction: row;}")}>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"HOME"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"SERVICE"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"NEWS"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"BLOG"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"CONTACT"}</a></li>
                        </ul>
                        <label class={classes!(toggle_button())}>
                            <input 
                                type="checkbox" 
                                class={classes!(toggle_slider())}
                                onchange={
                                    let dark_mode = dark_mode.clone();
                                    Callback::from(move |_| dark_mode.set(!*dark_mode))
                                }
                            checked={*dark_mode}/>  
                        </label>
                    </nav>
                    <main class={main_classes}>
                        <div class={classes!(center_styles())}>
                            <a href="https://github.com/Geothelphusa">
                                <img class={classes!(title_logo())} src={logo_path}/>
                            </a>
                        </div>

                        <h1>{"Welcome to Geothelphusa site !"}</h1>
                        <div class={classes!(center_styles())}>
                        <p class={css!("align-items:flex-end;")}>{if *dark_mode {"Dark"} else {"Light"}}</p>
                        </div>
                    </main>
                </div>
            </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use stylist::{style, yew::styled_component, StyleFragment};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub is_opened: bool,
}

#[styled_component(MenuButton)]
pub fn menu_button(props: &MenuButtonProps) -> Html {
    let style = style!(
        r#"
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        row-gap: 6px;

        &__line,
        &::before,
        &::after {
            content: "";
            width: 28px;
            height: 2px;
            background-color: #333333;
            transition: transform 0.3s, opacity 0.3s;
        }

        &.is-opened &__line {
            opacity: 0;
        }

        &.is-opened::before {
            transform: translateY(8px) rotate(45deg);
        }

        &.is-opened::after {
            transform: translateY(-8px) rotate(-45deg);
        }
        "#
    )
    .unwrap();

    let class = if props.is_opened {
        classes!("menu-button", "is-opened")
    } else {
        classes!("menu-button")
    };

    html! {
        <button id="menuButton" type="button" class={classes!(class, style.get_class_name().clone())} aria-labelledby="menuButtonLabel" onclick={props.onclick.clone()}>
            <StyleFragment r#style={style} />
            <span class="menu-button__line">
                <span id="menuButtonLabel" style="display: none">{"メニューボタン"}</span>
            </span>
        </button>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let is_menu_opened = use_state(|| false);

    let onclick = {
        let is_menu_opened = is_menu_opened.clone();
        Callback::from(move |_| {
            is_menu_opened.set(!*is_menu_opened);
        })
    };

    html! {
        <MenuButton onclick={onclick} is_opened={*is_menu_opened} />
    }
}
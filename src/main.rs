mod styles;

use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::*;

#[styled_component(App)]
fn app() -> Html {
const LOGO_PATH: &str = "https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg";
    let logo_path = LOGO_PATH;

    // モードの状態を保持する変数(初期値はライトモード)
    let dark_mode = use_state(|| false);

    let toggle_light: Callback<MouseEvent> = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // ライトモードに設定
            dark_mode.set(false);
        })
    };
    let toggle_dark: Callback<MouseEvent> = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dark_mode.set(true);
        })
    };

    let mut main_classes = Classes::new();
    main_classes.push(container_styles());
    if *dark_mode {
        main_classes.push(dark_mode_styles());
    } else {
        main_classes.push(light_mode_styles());
    };

    html! {
            <body class={classes!(base_styles())}>
                <div>
                    <nav class={classes!(nav_styles())}>
                        <ul class={css!("display: flex;")}>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"HOME"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"SERVICE"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"NEWS"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"BLOG"}</a></li>
                          <li class={classes!(li_none())}><a class={classes!(menu_items())} href="#">{"CONTACT"}</a></li>
                        </ul>
                        <button class={classes!(input_and_button())} type="submit" onclick={toggle_light}>{"Toggle Light Mode"}</button>
                        <button class={classes!(input_and_button())} type="submit" onclick={toggle_dark}>{"Toggle Dark Mode"}</button>
                    </nav>
                    <main class={main_classes}>
                        <div class={classes!(center_styles())}>
                            <a href="https://github.com/Geothelphusa">
                                <img class={classes!(title_logo())} src={logo_path}/>
                            </a>
                        </div>

                        <h1>{"Welcome to Geothelphusa site !"}</h1>
                        <div class={classes!(center_styles())}>
                            
                        </div>
                        <p>{if *dark_mode {"Dark Mode"} else {"Light Mode"}}</p>
                    </main>
                </div>
            </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

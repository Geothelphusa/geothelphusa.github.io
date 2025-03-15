mod styles;

use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::*;

#[styled_component(App)]
fn app() -> Html {
const LOGO_PATH: &str = "https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg";
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
                <div>
                    <nav class={classes!(nav_styles())}>
                        <ul class={css!("display: flex;")}>
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

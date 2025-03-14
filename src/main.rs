mod styles;

use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::*;

#[styled_component(App)]
fn app() -> Html {
const LOGO_PATH: &str = "./static/Geothelphusa.jpeg";
    let logo_path = LOGO_PATH;
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

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
        <global class={classes!(base_styles())}>
            <main class={main_classes}>
                <h1>{"Welcome to Geothelphusa site !"}</h1>

                <div class={classes!(center_styles())}>
                <a href="https://github.com/Geothelphusa">
                    <img src={logo_path}/>
                </a>
                </div>

                <div class={classes!(center_styles())}>
                <img src="./static/Geothelphusa.jpeg"/>
                </div>


                <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
                </div>

                // <div class={classes!(row_styles())}>
                    // <a class={classes!(a_tag())} href="https://tauri.app" target="_blank">
                    //     <img src="public/tauri.svg" class={classes!(logo())} alt="Tauri logo"/>
                    // </a>
                    // <a class={classes!(a_tag())} href="https://yew.rs" target="_blank">
                    //     <img src="public/yew.png" class={classes!(logo())} alt="Yew logo"/>
                    // </a>

                // </div>
                // <p class={classes!(center_styles())}>{"Click on the Tauri and Yew logos to learn more."}</p>

                // <form class={classes!(row_styles())} onsubmit={greet}>
                //     <input class={classes!(input_and_button())} id={classes!(greet_input())} ref={greet_input_ref} placeholder="Enter a name..." />
                //     <button class={classes!(input_and_button())} type="submit">{"Greet"}</button>
                // </form>
                // <p>{&*greet_msg}</p>

                <div class={classes!(center_styles())}>
<button class={classes!(input_and_button())} type="submit" onclick={toggle_light} aria-pressed={!*dark_mode}>{"Toggle Light Mode"}</button>
                    <button class={classes!(input_and_button())} type="submit" onclick={toggle_dark} aria-pressed={*dark_mode}>{"Toggle Dark Mode"}</button>
                </div>
                <p>{if *dark_mode {"Dark Mode"} else {"Light Mode"}}</p>
            </main>
        </global>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
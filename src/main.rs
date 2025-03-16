mod styles;
mod route;
mod components;

use stylist::{yew::styled_component, style};
use yew::{prelude::*, Renderer};
use yew_router::prelude::*;
use crate::styles::*;
use crate::route::*;
use crate::components::*;

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

    let is_menu_opened = use_state(|| false);

    let onclick = {
        let is_menu_opened_clone = is_menu_opened.clone();
        Callback::from(move |_| is_menu_opened_clone.set(!*is_menu_opened_clone))
    };

    let onclick_clone = onclick.clone();

        // モードの状態を保持する変数(初期値はライトモード)
    let dark_mode = use_state(|| true);
    let dark_mode_clone = dark_mode.clone();

    let mut main_classes = Classes::new();
    main_classes.push(container_styles());
    if *dark_mode {
        main_classes.push(dark_mode_styles());
    } else {
        main_classes.push(light_mode_styles());
    };

    html! {
        <main class={classes!(main_classes, base_styles())}>
            <div class={stylesheet}>
                <BrowserRouter>
                    <nav class={classes!(nav_styles())}>
                        <MenuButton onclick={onclick_clone} is_opened={*is_menu_opened} />
                    </nav>
                    <ul class={css!("display: flex; flex-direction: column; @media (min-width: 768px) {flex-direction: row;}")}>
                        { if *is_menu_opened {
                            html! {
                                <div class={classes!(overlay_style(), "is-opened")} onclick={onclick.clone()}>
                                    <div class={classes!(menu_style())} onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                        <ul class={classes!(menu_list_style())}>
                                            { for vec![
                                                (Route::About, "ABOUT"),
                                                (Route::Home, "HOME"),
                                                (Route::Service, "SERVICE"),
                                                (Route::News, "NEWS"),
                                                (Route::Blog, "BLOG"),
                                                (Route::Contact, "CONTACT"),
                                            ].into_iter().map(|(route, label)| html! {
                                                <li><Link<Route> to={route} classes={classes!(menu_items())}>{ label }</Link<Route>></li>
                                            }) }
                                        </ul>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </ul>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
    
                // ダークモード切り替え
                <label class={classes!(toggle_button())}>
                    <input 
                        type="checkbox" 
                        class={classes!(toggle_slider())}
                        oninput={Callback::from(move |_| dark_mode_clone.set(!*dark_mode_clone))}
                        checked={*dark_mode}
                    />  
                </label>
    
                // メインコンテンツ
                <div class={classes!(center_styles())}>
                    <a href="https://github.com/Geothelphusa">
                        <img class={classes!(title_logo())} src={logo_path}/>
                    </a>
                </div>
                <h1>{"Welcome to Geothelphusa site !"}</h1>
                <div class={classes!(center_styles())}>
                    <p class={css!("align-items:flex-end;")}>{ if *dark_mode { "Dark" } else { "Light" } }</p>
                </div>
            </div>
        </main>
    }
    
}

fn main() {
    Renderer::<App>::new().render();
}

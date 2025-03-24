mod styles;
mod route;
mod components;
mod home;
mod about;
mod blog;

use stylist::yew::styled_component;
use yew::{prelude::*, Renderer};
use yew_router::prelude::*;
use crate::styles::*;
use crate::route::*;
use crate::components::*;

#[styled_component(App)]
pub fn app() -> Html {

    let stylesheet = responsive_styles();

    let is_menu_opened = use_state(|| false);

    let onclick = {
        let is_menu_opened_clone = is_menu_opened.clone();
        Callback::from(move |_| is_menu_opened_clone.set(!*is_menu_opened_clone))
    };

    let onclick_clone = onclick.clone();

    // Brightness mode handling (init:dark)
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
                <HashRouter>
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
                                                (Route::Home, "HOME"),
                                                (Route::About, "ABOUT"),
                                                (Route::Service, "SERVICE"),
                                                (Route::News, "NEWS"),
                                                (Route::Blog, "BLOG"),
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
                </HashRouter>
    
                // Brightness mode switch
                <label class={classes!(toggle_button())}>
                    <input 
                        type="checkbox" 
                        class={classes!(toggle_slider())}
                        oninput={Callback::from(move |_| dark_mode_clone.set(!*dark_mode_clone))}
                        checked={*dark_mode}
                    />  
                </label>
            </div>
        </main>
    }
    
}

fn main() {
    Renderer::<App>::new().render();
}

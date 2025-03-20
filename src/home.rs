use yew::prelude::*;
use stylist::yew::styled_component;

use crate::styles::*;
use crate::components::*;

#[styled_component(Home)]
pub fn home() -> Html {
    let logo_path = LOGO_PATH;
    html! (
        <>
            // main containts
            <div class={classes!(center_styles())}>
                <a href="https://github.com/Geothelphusa">
                    <img class={classes!(title_logo())} src={logo_path}/>
                </a>
            </div>
            <h1>{"Welcome to Geothelphusa site !"}</h1>
        </>
    )
}

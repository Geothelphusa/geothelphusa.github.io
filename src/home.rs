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
            <h1>{"The menu on the upper left links to each page."}</h1>
            <hr style="width: 90%;"/>
            <h1>{"ABOUT: Naming of this organization"}</h1>
            <h1>{"SERVICE: Applications offered by this organization"}</h1>
            <h1>{"NEWS: Mainly application release articles, etc."}</h1>
            <h1>{"BLOG: Will mainly upload development articles, etc."}</h1>
            <hr style="width: 90%;"/>
            <h2>{"This site is open source software under the Apache License 2.0."}</h2>
            <h2>{"Contributions are always welcome!"}</h2>
        </>
    )
}

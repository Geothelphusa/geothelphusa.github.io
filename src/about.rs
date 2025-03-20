use yew::prelude::*;
use stylist::yew::styled_component;

use crate::styles::*;

#[styled_component(About)]
pub fn about() -> Html {
    html! (
        <>
            <main>
                <div class={classes!(center_styles())}>
                    <h1>{"What is Geothelphusa?"}</h1>
                </div>
                <p>{"Geothelphusa is a genus of freshwater crabs in the family Potamidae, native to Taiwan."}</p>
            </main>
        </>

    )
}

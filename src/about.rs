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
                <hr style="width: 90%;"/>
                <h2>{"We exist to provide OSS as an option for manufacturing workers in the digital transformation of their workplaces."}</h2>
                <hr style="width: 90%;"/>
                    <p><b>{"Geothelphusa is a genus of freshwater crabs in the family Potamidae, native to Taiwan."}</b></p>
                    <p><b>{"This name was chosen because Geothelphusa is the most familiar crab species in Japanese rivers, and we wanted to be associated with it."}</b></p>
                    <p><b>{""}</b></p>
                    <p><b>{""}</b></p>
                    
            </main>
        </>

    )
}

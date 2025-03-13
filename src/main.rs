mod rm_integrity;
mod refresh_update;

use yew::prelude::*;
use rm_integrity::remove_integrity_attribute;
use refresh_update::update_integrity_attributes;

#[function_component]
fn App() -> Html {
    let _ = remove_integrity_attribute();
    let _ = update_integrity_attributes();
    
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
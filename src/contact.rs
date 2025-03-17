use crate::webhook;

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn get_webhook_url() -> &'static str {
    webhook::WEBHOOK_URL
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let name = use_state(|| String::new());
    let message = use_state(|| String::new());

    let on_submit = {
        let name = name.clone();
        let message = message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.clone();
            let message = message.clone();

            spawn_local(async move {
                let webhook_url = get_webhook_url();

                let payload = serde_json::json!({
                    "content": format!("**お問い合わせ**\n**名前:** {}\n**メッセージ:** {}", *name, *message)
                });

                let _ = Request::post(webhook_url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .unwrap()
                    .send()
                    .await;
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <label for="name">{ "名前: " }</label>
            <input
                id="name"
                type="text"
                value={(*name).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    name.set(input.value());
                })}
            />

            <label for="message">{ "メッセージ: " }</label>
            <textarea
                id="message"
                value={(*message).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    message.set(input.value());
                })}
            ></textarea>

            <button type="submit">{ "送信" }</button>
        </form>
    }
}

use yew::prelude::*;
use stylist::yew::styled_component;

use crate::styles::*;

#[styled_component(Blog)]
pub fn blog() -> Html {
  html! (
    <>
        <main>
            <div class={classes!(container_styles(), center_styles())}>
              <h1>{"Blog"}</h1>
              <div id="html-part" class={classes!(link_card_style())}>
<a class="link-card" href="https://gist.github.com/YoshitakaNaraoka/9c1b9a03f76e54d4038b4321dcf62114" rel="noopener noreferrer">
                  <div class="link-card-content">
                    <div class="link-card-image" id="grid-content-1">
<img id="thumbnail-image-code" src="https://github.githubassets.com/assets/gist-og-image-54fd7dc0713e.png" alt="Geothelphusa philosophy thumbnail" />
                    </div>
                    <div class="link-card-text" id="grid-content-2">
                      <p class="link-card-title">{"Geothelphusa_philosophy.md"}</p>
                      <p class="link-card-description">{"Geothelphusa philosophy in Japanese"}</p>
                      <div class="link-card-domain">
                        <img id="favicon-image-code" src="https://github.githubassets.com/favicons/favicon.svg" />
                        <p>{"https://gist.github.com/YoshitakaNaraoka/9c1b9a03f76e54d4038b4321dcf62114"}</p>
                      </div>
                    </div>
                  </div>
                </a>
              </div>
            </div>
        </main>
    </>
)
}

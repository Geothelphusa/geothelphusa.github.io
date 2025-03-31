use yew::prelude::*;
use stylist::yew::styled_component;

use crate::styles::*;

#[styled_component(Service)]
pub fn service() -> Html {
    html! (
        <>
            <main>
              <div class={classes!(container_styles(), center_styles())}>
                <h1>{"Service"}</h1>
                  <div id="html-part" class={classes!(link_card_style())} > 
                    <a class="link-card" href="https://github.com/Geothelphusa/millmill" rel="noopener noreferrer" >
                      <div class="link-card-content">
                        <div class="link-card-image" id="grid-content-1">
                          <img id="thumbnail-image-code" src="https://opengraph.githubassets.com/c04f0dc382efc4bc10339ce5e5683c3050b3cf905ce0de651e208b493b3dcea2/Geothelphusa/millmill" />
                        </div> 
                        <div class="link-card-text" id="grid-content-2">
                          <p class="link-card-title">{"Geothelphusa/millmill"}</p>
                        </div>
                        <div id="grid-content-3">
                          <p class="link-card-description">{"Monitoring machine cutting."}</p>
                          <div class="link-card-domain">
                            <img id="favicon-image-code" src="https://github.githubassets.com/favicons/favicon.svg" />
                            <p>{"https://github.com/Geothelphusa/millmill"}</p>
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

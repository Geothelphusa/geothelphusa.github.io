use yew_router::prelude::*;
use crate::home::*;
use crate::about::*;
use crate::blog::*;
use crate::service::*;


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/service")]
    Service,
    #[at("/news")]
    News,
    #[at("/blog")]
    Blog,
}

use yew::prelude::*;

#[function_component(NotFound)]
fn not_found() -> Html {
    html! { <h1>{ "404 Not Found" }</h1> }
}

pub fn switch(route: Route) -> Html {
  match route {
      Route::Home => html! { <Home /> },
      Route::About => html! { <About /> },
      Route::Service => html! { <Service /> },
      Route::News => html! { <h1>{ "News Page" }</h1> },
      Route::Blog => html! { <Blog /> },
  }
}

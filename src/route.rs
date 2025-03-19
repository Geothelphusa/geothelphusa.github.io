use yew_router::prelude::*;
use crate::home::*;


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
    #[not_found]
    #[at("/404")]
    NotFound,
}

use yew::prelude::*;


#[function_component(About)]
fn about() -> Html {
    html! { <h1>{ "About Page" }</h1> }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! { <h1>{ "404 Not Found" }</h1> }
}

pub fn switch(route: Route) -> Html {
  match route {
      Route::Home => html! { <Home /> },
      Route::About => html! { <About /> },
      Route::Service => html! { <h1>{ "Service Page" }</h1> },
      Route::News => html! { <h1>{ "News Page" }</h1> },
      Route::Blog => html! { <h1>{ "Blog Page" }</h1> },
      Route::NotFound => html! { <NotFound /> },
  }
}

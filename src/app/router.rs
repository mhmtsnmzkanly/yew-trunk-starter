use super::pages::*;
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum RoutableList {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn route_page(routes: &RoutableList) -> Html {
    match routes.clone() {
        RoutableList::Index => {
            html! { <index::Index  /> }
        }
        RoutableList::NotFound => {
            html! { <not_found::NotFound /> }
        }
    }
}

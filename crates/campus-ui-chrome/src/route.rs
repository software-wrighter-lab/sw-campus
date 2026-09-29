use yew_router::prelude::Routable;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/campus")]
    Campus,
    #[at("/campus/*path")]
    Place { path: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

use crate::Route;
use campus_model::{Catalog, Place, PlaceId, Status, ancestors};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct DirectoryProps {
    pub place: Place,
    pub catalog: Catalog,
}

#[function_component(Directory)]
pub fn directory(props: &DirectoryProps) -> Html {
    html! { <section class="directory"><p class="kind">{format!("{:?}", props.place.kind)}</p><h1>{&props.place.title}</h1><p class="tagline">{&props.place.tagline}</p><p>{&props.place.summary}</p><ul>{ for props.place.children.iter().filter_map(|id| directory_entry(&props.catalog, id)) }</ul></section> }
}

fn directory_entry(catalog: &Catalog, id: &PlaceId) -> Option<Html> {
    let place = catalog.get(id)?;
    let chain = ancestors(catalog, id)?;
    let path = chain
        .iter()
        .skip(1)
        .map(|item| item.id.0.clone())
        .collect::<Vec<_>>()
        .join("/");
    let status = match place.status {
        Status::Open => "Open",
        Status::Placeholder => "Opening soon",
        Status::ComingSoon => "Coming soon",
    };
    Some(
        html! { <li><Link<Route> to={Route::Place { path }}>{&place.title}</Link<Route>><span class="status">{status}</span><p>{&place.tagline}</p></li> },
    )
}

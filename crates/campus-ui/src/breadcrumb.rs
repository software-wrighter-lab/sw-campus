use crate::Route;
use campus_model::{Catalog, PlaceId, ancestors};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct BreadcrumbProps {
    pub catalog: Catalog,
    pub place: PlaceId,
}

#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    let places = ancestors(&props.catalog, &props.place).unwrap_or_default();
    html! {
        <nav class="breadcrumb" aria-label="Where you are">
            { for places.iter().enumerate().map(|(index, place)| {
                let path = places.iter().skip(1).take(index).chain(std::iter::once(place)).map(|item| item.id.0.clone()).collect::<Vec<_>>().join("/");
                let route = if index == 0 { Route::Campus } else { Route::Place { path } };
                html! { <span class="breadcrumb-item"><Link<Route> to={route}>{&place.title}</Link<Route>>{ if index + 1 < places.len() { html! { " / " } } else { Html::default() } }</span> }
            }) }
        </nav>
    }
}

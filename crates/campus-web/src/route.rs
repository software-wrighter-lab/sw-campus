use campus_model::{Catalog, Place, resolve};
use campus_ui::{Directory, NotFound, Route};
use yew::prelude::*;

pub fn switch(route: Route, catalog: &Catalog) -> Html {
    let place = match route {
        Route::Home | Route::Campus => catalog.root(),
        Route::Place { path } => place_from_path(catalog, &path),
        Route::NotFound => None,
    };
    match place {
        Some(place) => html! { <Directory place={place.clone()} catalog={catalog.clone()} /> },
        None => html! { <NotFound /> },
    }
}

fn place_from_path<'a>(catalog: &'a Catalog, path: &str) -> Option<&'a Place> {
    let parts: Vec<_> = path.split('/').filter(|part| !part.is_empty()).collect();
    if parts.first().copied() == Some("campus") {
        resolve(catalog, &parts[1..])
    } else {
        resolve(catalog, &parts)
    }
}

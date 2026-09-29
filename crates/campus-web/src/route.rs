use campus_model::{Catalog, Place, resolve};
use campus_scene::Scenes;
use campus_ui_chrome::{Directory, NotFound, Route};
use campus_ui_exhibits::ExhibitPage;
use campus_ui_scene::SceneView;
use yew::prelude::*;
use yew_router::prelude::Navigator;

type CatalogState = UseStateHandle<Catalog>;

pub fn switch(route: &Route, catalog: &CatalogState, navigator: Option<Navigator>) -> Html {
    let Some(place) = place(route, catalog) else {
        return html! { <NotFound /> };
    };
    if leaf(place) {
        return html! { <ExhibitPage place={place.clone()} catalog={(**catalog).clone()} base={crate::base::basename()} /> };
    }
    let directory = html! { <Directory place={place.clone()} catalog={(**catalog).clone()} /> };
    let Some(scene_id) = place.scene.clone() else {
        return directory;
    };
    let scenes = Scenes::embedded().expect("embedded scenes");
    let Some(scene) = scenes.get(&scene_id) else {
        return directory;
    };
    let on_navigate = Callback::from(move |path: String| {
        if let Some(navigator) = navigator.clone() {
            navigator.push(&Route::Place { path });
        }
    });
    html! { <><SceneView scene={scene.clone()} catalog={(**catalog).clone()} base={crate::base::basename()} on_navigate={on_navigate} />{directory}</> }
}

pub fn place<'a>(route: &Route, catalog: &'a Catalog) -> Option<&'a Place> {
    match route {
        Route::Home | Route::Campus => catalog.root(),
        Route::Place { path } => place_from_path(catalog, path),
        Route::NotFound => None,
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

fn leaf(place: &Place) -> bool {
    place.scene.is_none() && place.children.is_empty()
}

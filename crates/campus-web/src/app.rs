use crate::base::Footer;
use campus_model::{Catalog, PlaceId};
use campus_scene::Scenes;
use campus_ui_chrome::{Breadcrumb, Directory, NotFound, Route};
use campus_ui_scene::SceneView;
use yew::prelude::*;
use yew_router::prelude::{BrowserRouter, Navigator, Switch, use_navigator};

#[function_component(App)]
pub fn app() -> Html {
    let catalog = use_state(|| Catalog::embedded().expect("embedded campus catalog"));
    html! { <ContextProvider<UseStateHandle<Catalog>> context={catalog.clone()}><BrowserRouter basename={crate::base::basename()}><RouterShell /></BrowserRouter></ContextProvider<UseStateHandle<Catalog>>> }
}

#[function_component(RouterShell)]
fn router_shell() -> Html {
    let catalog = use_context::<UseStateHandle<Catalog>>().expect("catalog context");
    let navigator = use_navigator();
    html! { <Switch<Route> render={move |route| shell(&route, &catalog, navigator.clone())} /> }
}

fn shell(route: &Route, catalog: &UseStateHandle<Catalog>, navigator: Option<Navigator>) -> Html {
    let current = match &route {
        Route::Home | Route::Campus => catalog.root().map(|place| place.id.clone()),
        Route::Place { path } => path
            .split('/')
            .rfind(|part| !part.is_empty())
            .map(|id| PlaceId(id.to_owned())),
        Route::NotFound => None,
    };
    html! {
        <main class="campus-shell">
            <header><a href=".">{"Software Wrighter Research Campus"}</a></header>
            <main class="campus-content">
                { current.map(|id| html! { <Breadcrumb catalog={(**catalog).clone()} place={id} /> }).unwrap_or_default() }
                {render_content(route, catalog, navigator)}
            </main>
            <Footer />
        </main>
    }
}

fn render_content(
    route: &Route,
    catalog: &UseStateHandle<Catalog>,
    navigator: Option<Navigator>,
) -> Html {
    let Some(place) = crate::route::place(route, catalog) else {
        return html! { <NotFound /> };
    };
    let directory = html! { <Directory place={place.clone()} catalog={(**catalog).clone()} /> };
    let Some(scene_id) = place.scene.clone() else {
        return crate::route::switch(route, catalog);
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

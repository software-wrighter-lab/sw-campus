use crate::base::Footer;
use campus_model::{Catalog, PlaceId};
use campus_ui_chrome::{Breadcrumb, Route};
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
    let current = match route {
        Route::Home | Route::Campus => catalog.root().map(|place| place.id.clone()),
        Route::Place { path } => path
            .split('/')
            .rfind(|part| !part.is_empty())
            .map(|id| PlaceId(id.to_owned())),
        Route::NotFound => None,
    };
    html! { <main class="campus-shell"><header><a href=".">{"Software Wrighter Research Campus"}</a></header><main class="campus-content">{ current.map(|id| html! { <Breadcrumb catalog={(**catalog).clone()} place={id} /> }).unwrap_or_default() }{crate::route::switch(route, catalog, navigator)}</main><Footer /></main> }
}

use campus_model::{Catalog, PlaceId};
use campus_ui::{Breadcrumb, Route};
use yew::prelude::*;
use yew_router::prelude::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    let catalog = use_state(|| Catalog::embedded().expect("embedded campus catalog"));
    html! {
        <ContextProvider<UseStateHandle<Catalog>> context={catalog.clone()}>
            <BrowserRouter basename={crate::base::basename()}>
                <Switch<Route> render={move |route| shell(route, &catalog)} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<Catalog>>>
    }
}

fn shell(route: Route, catalog: &UseStateHandle<Catalog>) -> Html {
    let current = current_id(&route, catalog);
    let content = crate::route::switch(route, catalog);
    html! {
        <main class="campus-shell">
            <header><a href=".">{"Software Wrighter Research Campus"}</a></header>
            { current.map(|id| html! { <Breadcrumb catalog={(**catalog).clone()} place={id} /> }).unwrap_or_default() }
            <main class="campus-content">{content}</main>
            <Footer />
        </main>
    }
}

fn current_id(route: &Route, catalog: &Catalog) -> Option<PlaceId> {
    match route {
        Route::Home | Route::Campus => catalog.root().map(|place| place.id.clone()),
        Route::Place { path } => path
            .split('/')
            .rfind(|part| !part.is_empty())
            .map(|id| PlaceId(id.to_owned())),
        Route::NotFound => None,
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! { <footer><span>{"Copyright (c) 2026 Michael A Wright"}</span><a href="LICENSE">{"MIT License"}</a><a href="https://github.com/software-wrighter-lab/sw-campus">{"Repository: software-wrighter-lab/sw-campus"}</a><span>{"Build Host: "}{env!("BUILD_HOST")}</span><span>{"Build Commit: "}{env!("BUILD_SHA")}</span><span>{"Build Time: "}{env!("BUILD_TIMESTAMP")}</span></footer> }
}

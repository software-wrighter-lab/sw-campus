use crate::base::Footer;
use campus_docent::Reply;
use campus_model::{Catalog, PlaceId};
use campus_ui_chrome::{Breadcrumb, Route};
use campus_ui_docent::DocentPanel;
use yew::prelude::*;
use yew_router::prelude::{BrowserRouter, Navigator, Switch, use_navigator};

#[function_component(App)]
pub fn app() -> Html {
    let catalog = use_state(|| Catalog::embedded().expect("embedded campus catalog"));
    let docent_reply = use_state(|| None::<Reply>);
    html! { <ContextProvider<UseStateHandle<Catalog>> context={catalog.clone()}><BrowserRouter basename={crate::base::basename()}><RouterShell docent_reply={docent_reply} /></BrowserRouter></ContextProvider<UseStateHandle<Catalog>>> }
}

#[derive(Properties, PartialEq)]
struct RouterShellProps {
    docent_reply: UseStateHandle<Option<Reply>>,
}

#[function_component(RouterShell)]
fn router_shell(props: &RouterShellProps) -> Html {
    let catalog = use_context::<UseStateHandle<Catalog>>().expect("catalog context");
    let docent_reply = props.docent_reply.clone();
    let on_answer = {
        let docent_reply = docent_reply.clone();
        Callback::from(move |reply| docent_reply.set(reply))
    };
    html! { <Switch<Route> render={move |route: Route| html! { <RouteShell route={route.clone()} catalog={(*catalog).clone()} docent_reply={docent_reply.clone()} on_answer={on_answer.clone()} /> }} /> }
}

#[derive(Properties, PartialEq)]
struct RouteShellProps {
    route: Route,
    catalog: Catalog,
    docent_reply: UseStateHandle<Option<Reply>>,
    on_answer: Callback<Option<Reply>>,
}

#[function_component(RouteShell)]
fn route_shell(props: &RouteShellProps) -> Html {
    {
        let docent_reply = props.docent_reply.clone();
        use_effect_with(props.route.clone(), move |_| {
            docent_reply.set(None);
            || ()
        });
    }
    let navigator = use_navigator();
    shell(
        &props.route,
        &props.catalog,
        navigator,
        &props.docent_reply,
        props.on_answer.clone(),
    )
}

fn shell(
    route: &Route,
    catalog: &Catalog,
    navigator: Option<Navigator>,
    docent_reply: &UseStateHandle<Option<Reply>>,
    on_answer: Callback<Option<Reply>>,
) -> Html {
    let current = match route {
        Route::Home | Route::Campus => catalog.root().map(|place| place.id.clone()),
        Route::Place { path } => path
            .split('/')
            .rfind(|part| !part.is_empty())
            .map(|id| PlaceId(id.to_owned())),
        Route::NotFound => None,
    };
    html! { <main class="campus-shell"><header><a href=".">{"Software Wrighter Research Campus"}</a></header><main class="campus-content">{ current.map(|id| html! { <Breadcrumb catalog={catalog.clone()} place={id} /> }).unwrap_or_default() }{crate::route::switch(route, catalog, navigator, (**docent_reply).clone())}</main><DocentPanel catalog={catalog.clone()} on_answer={on_answer} /><Footer /></main> }
}

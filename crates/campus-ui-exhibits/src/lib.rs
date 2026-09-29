use campus_model::{Catalog, LinkKind, Place, Status};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExhibitPageProps {
    pub place: Place,
    pub catalog: Catalog,
    pub base: String,
}

#[function_component(ExhibitPage)]
pub fn exhibit_page(props: &ExhibitPageProps) -> Html {
    let run = props
        .place
        .links
        .iter()
        .find(|link| link.kind == LinkKind::Run);
    let status = matches!(props.place.status, Status::Placeholder | Status::ComingSoon)
        .then_some("Opening soon");
    html! {
        <article class="exhibit-page">
            <p class="kind">{format!("{:?}", props.place.kind)}</p>
            <h1>{&props.place.title}</h1>
            <p class="tagline">{&props.place.tagline}</p>
            { for status.map(|label| html! { <p class="status">{label}</p> }) }
            <p>{&props.place.summary}</p>
            { for run.map(|link| html! { <a class="run-button" href={link.url.clone()} target="_blank" rel="noopener">{"Run exhibit"}</a> }) }
            <nav class="exhibit-links" aria-label="Exhibit links">
                { for props.place.links.iter().filter(|link| link.kind != LinkKind::Run).map(|link| html! { <a href={link.url.clone()} target="_blank" rel="noopener">{format!("{}: {}", link_label(link.kind), link.label)}</a> }) }
            </nav>
            { children(props) }
        </article>
    }
}

fn link_label(kind: LinkKind) -> &'static str {
    match kind {
        LinkKind::Source => "Source",
        LinkKind::Docs => "Docs",
        LinkKind::Related => "Related",
        LinkKind::Run => "Run",
    }
}

fn children(props: &ExhibitPageProps) -> Html {
    if props.place.children.is_empty() {
        return Html::default();
    }
    html! { <section><h2>{"Related exhibits"}</h2><ul>{ for props.place.children.iter().filter_map(|id| props.catalog.get(id)).map(|child| html! { <li><a href={format!("{}/campus/{}", props.base, child.id.0)}>{&child.title}</a><span class="status">{format!("{:?}", child.status)}</span></li> }) }</ul></section> }
}

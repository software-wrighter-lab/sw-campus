use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CaptionCardProps {
    pub title: String,
    pub tagline: String,
    pub children: String,
    pub status: Option<&'static str>,
    pub anchor: Option<(f32, f32)>,
}

#[function_component(CaptionCard)]
pub fn caption_card(props: &CaptionCardProps) -> Html {
    let style = props
        .anchor
        .map(|(x, y)| format!("--caption-x: {x}px; --caption-y: {y}px;"));
    html! { <div class="caption-card" style={style}><strong>{&props.title}</strong><p>{&props.tagline}</p>{ for props.status.iter().map(|status| html! { <em>{status}</em> }) }{ if props.children.is_empty() { Html::default() } else { html! { <small>{&props.children}</small> } } }</div> }
}

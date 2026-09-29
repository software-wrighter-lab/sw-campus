use campus_model::{Catalog, PlaceId, ancestors};
use campus_scene::{Hotspot, Scene};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HotspotLayerProps {
    pub scene: Scene,
    pub catalog: Catalog,
    pub base: String,
    pub hovered: Option<PlaceId>,
    pub on_hover: Callback<Option<PlaceId>>,
    pub on_navigate: Callback<String>,
}

#[function_component(HotspotLayer)]
pub fn hotspot_layer(props: &HotspotLayerProps) -> Html {
    html! { <g class="hotspot-layer">{ for props.scene.hotspots.iter().map(|hotspot| hotspot_link(hotspot, props)) }</g> }
}

fn hotspot_link(hotspot: &Hotspot, props: &HotspotLayerProps) -> Html {
    let place = props.catalog.get(&hotspot.place);
    let title = place.map_or_else(|| hotspot.place.0.clone(), |place| place.title.clone());
    let path = ancestors(&props.catalog, &hotspot.place).map(|chain| {
        chain
            .iter()
            .skip(1)
            .map(|place| place.id.0.clone())
            .collect::<Vec<_>>()
            .join("/")
    });
    let href = path.as_ref().map_or_else(
        || "#".to_owned(),
        |path| format!("{}/campus/{}", props.base, path),
    );
    let id = hotspot.place.clone();
    let on_navigate = props.on_navigate.clone();
    let route = path.unwrap_or_default();
    let onclick = Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        on_navigate.emit(route.clone());
    });
    let hover_id = id.clone();
    let focus_id = id.clone();
    let hover_callback = props.on_hover.clone();
    let focus_callback = props.on_hover.clone();
    let leave_callback = props.on_hover.clone();
    let blur_callback = props.on_hover.clone();
    html! { <a href={href} role="link" tabindex="0" aria-label={title.clone()} class={classes!("hotspot", (props.hovered.as_ref() == Some(&id)).then_some("hotspot-active"))} onclick={onclick} onmouseover={Callback::from(move |_| hover_callback.emit(Some(hover_id.clone())))} onfocus={Callback::from(move |_| focus_callback.emit(Some(focus_id.clone())))} onmouseout={Callback::from(move |_| leave_callback.emit(None))} onblur={Callback::from(move |_| blur_callback.emit(None))}><polygon points={points(&hotspot.shape)} /></a> }
}

fn points(shape: &campus_scene::Shape) -> String {
    shape
        .points()
        .iter()
        .map(|(x, y)| format!("{x},{y}"))
        .collect::<Vec<_>>()
        .join(" ")
}

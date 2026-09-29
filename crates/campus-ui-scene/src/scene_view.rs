use crate::{CaptionCard, HotspotLayer};
use campus_model::{Catalog, PlaceId};
use campus_scene::Scene;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneViewProps {
    pub scene: Scene,
    pub catalog: Catalog,
    pub base: String,
    pub on_navigate: Callback<String>,
}

#[function_component(SceneView)]
pub fn scene_view(props: &SceneViewProps) -> Html {
    let hovered = use_state(|| None::<PlaceId>);
    let on_hover = {
        let hovered = hovered.clone();
        Callback::from(move |id| hovered.set(id))
    };
    let selected = (*hovered).clone();
    html! { <div class="scene-view"><svg viewBox={format!("0 0 {} {}", props.scene.width, props.scene.height)} preserveAspectRatio="xMidYMid meet" role="img" aria-label="Campus scene">
    <image href={format!("assets/{}", props.scene.image)} x="0" y="0" width={props.scene.width.to_string()} height={props.scene.height.to_string()} />
    <HotspotLayer scene={props.scene.clone()} catalog={props.catalog.clone()} base={props.base.clone()} hovered={selected.clone()} on_hover={on_hover} on_navigate={props.on_navigate.clone()} />
    </svg>{ selected.and_then(|id| caption(&props.catalog, &id, &props.scene)) }</div> }
}

fn caption(catalog: &Catalog, id: &PlaceId, scene: &Scene) -> Option<Html> {
    let place = catalog.get(id)?;
    let hotspot = scene.hotspots.iter().find(|hotspot| &hotspot.place == id)?;
    let children = place
        .children
        .iter()
        .filter_map(|child| catalog.get(child))
        .take(4)
        .map(|child| child.title.as_str())
        .collect::<Vec<_>>()
        .join(" - ");
    let status = matches!(
        place.status,
        campus_model::Status::Placeholder | campus_model::Status::ComingSoon
    )
    .then_some("Opening soon");
    Some(
        html! { <CaptionCard title={place.title.clone()} tagline={place.tagline.clone()} children={children} status={status} anchor={hotspot.shape.centroid()} /> },
    )
}

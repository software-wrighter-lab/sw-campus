use crate::{CaptionCard, HotspotEditor, HotspotLayer};
use campus_docent::Reply;
use campus_model::{Catalog, PlaceId};
use campus_scene::Scene;
use campus_ui_docent::PinLayer;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SceneViewProps {
    pub scene: Scene,
    pub catalog: Catalog,
    pub base: String,
    pub on_navigate: Callback<String>,
    pub docent_reply: Option<Reply>,
}

#[function_component(SceneView)]
pub fn scene_view(props: &SceneViewProps) -> Html {
    let edit_mode = edit_mode();
    let hovered = use_state(|| None::<PlaceId>);
    let on_hover = {
        let hovered = hovered.clone();
        Callback::from(move |id| hovered.set(id))
    };
    let parent = parent_path(&props.catalog, &props.scene.place);
    let on_navigate = props.on_navigate.clone();
    use_effect_with((), move |()| {
        let window = web_sys::window().expect("window available");
        let listener = EventListener::new(&window, "keydown", move |event| {
            let Some(keyboard) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
                return;
            };
            let path = match keyboard.key().as_str() {
                "Escape" => Some(parent.clone()),
                "h" | "H" => Some(String::new()),
                _ => None,
            };
            if let Some(path) = path {
                on_navigate.emit(path);
            }
        });
        move || drop(listener)
    });
    let selected = (*hovered).clone();
    html! { <div class="scene-view" key={props.scene.id.clone()}><svg viewBox={format!("0 0 {} {}", props.scene.width, props.scene.height)} preserveAspectRatio="xMidYMid meet" role="img" aria-label="Campus scene">
    <title>{ format!("{} scene", props.scene.place.0) }</title>
    <image href={format!("assets/{}", props.scene.image)} x="0" y="0" width={props.scene.width.to_string()} height={props.scene.height.to_string()} role="img" aria-label={format!("{} painted scene", props.scene.place.0)} />
    <HotspotLayer scene={props.scene.clone()} catalog={props.catalog.clone()} base={props.base.clone()} hovered={selected.clone()} on_hover={on_hover} on_navigate={props.on_navigate.clone()} />
    <PinLayer scene={props.scene.clone()} catalog={props.catalog.clone()} base={props.base.clone()} reply={props.docent_reply.clone()} on_navigate={props.on_navigate.clone()} />
    { edit_mode.then(|| html! { <HotspotEditor width={props.scene.width} height={props.scene.height} /> }) }
    </svg>{ selected.and_then(|id| caption(&props.catalog, &id, &props.scene)) }</div> }
}

fn edit_mode() -> bool {
    web_sys::window()
        .and_then(|window| window.location().search().ok())
        .is_some_and(|search| {
            search
                .trim_start_matches('?')
                .split('&')
                .any(|part| part == "edit" || part.starts_with("edit="))
        })
}

fn parent_path(catalog: &Catalog, place: &PlaceId) -> String {
    let Some(chain) = campus_model::ancestors(catalog, place) else {
        return String::new();
    };
    chain
        .iter()
        .skip(1)
        .take(chain.len().saturating_sub(2))
        .map(|item| item.id.0.clone())
        .collect::<Vec<_>>()
        .join("/")
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

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{MouseEvent, SvgElement, SvgsvgElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HotspotEditorProps {
    pub width: u32,
    pub height: u32,
}

#[function_component(HotspotEditor)]
pub fn hotspot_editor(props: &HotspotEditorProps) -> Html {
    let vertices = use_state(Vec::<(f32, f32)>::new);
    let closed = use_state(|| false);
    {
        let vertices = vertices.clone();
        let closed = closed.clone();
        use_effect_with((*vertices).clone(), move |current| {
            let current = current.clone();
            let window = web_sys::window().expect("window available");
            let listener = gloo_events::EventListener::new(&window, "keydown", move |event| {
                let Some(keyboard) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
                    return;
                };
                match keyboard.key().as_str() {
                    "Backspace" => {
                        let mut next = current.clone();
                        next.pop();
                        vertices.set(next);
                        closed.set(false);
                        keyboard.prevent_default();
                    }
                    "Escape" => {
                        vertices.set(Vec::new());
                        closed.set(false);
                    }
                    "Enter" if current.len() >= 3 => {
                        web_sys::console::log_1(&JsValue::from_str(&ron_for(&current)));
                        closed.set(true);
                    }
                    _ => {}
                }
            });
            move || drop(listener)
        });
    }
    let on_click = {
        let vertices = vertices.clone();
        let closed = closed.clone();
        let width = props.width;
        let height = props.height;
        Callback::from(move |event: MouseEvent| {
            let Some(svg) = event
                .current_target()
                .and_then(|target| target.dyn_into::<SvgElement>().ok())
                .and_then(|element| element.owner_svg_element())
            else {
                return;
            };
            let Some((x, y)) = scene_point(&svg, &event, width, height) else {
                return;
            };
            let mut next = if *closed {
                Vec::new()
            } else {
                (*vertices).clone()
            };
            next.push((x, y));
            vertices.set(next);
            closed.set(false);
        })
    };
    let point_string = points(&vertices);
    let shape = if *closed {
        html! { <polygon points={point_string.clone()} class="hotspot-editor-closed" /> }
    } else {
        html! { <polyline points={point_string.clone()} class="hotspot-editor-line" /> }
    };
    html! {
        <g class="hotspot-editor">
            { shape }
            { for vertices.iter().enumerate().map(|(index, (x, y))| html! {
                <circle cx={x.to_string()} cy={y.to_string()} r="7" class="hotspot-editor-point">
                    <title>{ format!("{}: {:.1}, {:.1}", index + 1, x, y) }</title>
                </circle>
            }) }
            <rect width={props.width.to_string()} height={props.height.to_string()} fill="transparent" onclick={on_click} />
        </g>
    }
}

#[allow(clippy::cast_precision_loss)]
fn scene_point(
    svg: &SvgsvgElement,
    event: &MouseEvent,
    width: u32,
    height: u32,
) -> Option<(f32, f32)> {
    let matrix = svg.get_screen_ctm()?.inverse().ok()?;
    let client_x = event.client_x() as f32;
    let client_y = event.client_y() as f32;
    let x = matrix.a() * client_x + matrix.c() * client_y + matrix.e();
    let y = matrix.b() * client_x + matrix.d() * client_y + matrix.f();
    Some((x.clamp(0.0, width as f32), y.clamp(0.0, height as f32)))
}

fn points(vertices: &[(f32, f32)]) -> String {
    vertices
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn ron_for(vertices: &[(f32, f32)]) -> String {
    let points = vertices
        .iter()
        .map(|(x, y)| format!("({x:.1}, {y:.1})"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("(place: \"TODO\", shape: Polygon([{points}]))")
}

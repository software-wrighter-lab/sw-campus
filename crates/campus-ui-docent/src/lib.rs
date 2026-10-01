use campus_docent::{Outcome, Reply, ask};
use campus_model::{Catalog, PlaceId, ancestors};
use campus_scene::Scene;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DocentPanelProps {
    pub catalog: Catalog,
    pub on_answer: Callback<Option<Reply>>,
}

#[function_component(DocentPanel)]
pub fn docent_panel(props: &DocentPanelProps) -> Html {
    let open = use_state(|| false);
    let question = use_state(String::new);
    let reply = use_state(|| None::<Reply>);
    let submit = submit_callback(props, &question, &reply);
    let on_input = {
        let question = question.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                question.set(input.value());
            }
        })
    };
    let close = {
        let open = open.clone();
        Callback::from(move |_| open.set(false))
    };
    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };
    html! {
        <>
            <button class="docent-button" type="button" aria-label="Open campus Docent" onclick={toggle}>{"Docent"}</button>
            { open.then(|| html! {
                <aside class="docent-dialog" role="dialog" aria-label="Campus Docent" aria-modal="false">
                    <header class="docent-dialog-head"><div><strong>{"Campus Docent"}</strong><small>{"Ask where to go."}</small></div><button type="button" aria-label="Close Docent" onclick={close}>{"×"}</button></header>
                    <div class="docent-transcript" aria-live="polite">{ reply.as_ref().map_or_else(|| html! { <p class="docent-hint">{"Try “where is the console with toggle switches?”"}</p> }, reply_view) }</div>
                    <form onsubmit={submit}>
                        <label for="docent-question">{"Ask the Docent"}</label>
                        <div class="docent-input-row"><input id="docent-question" value={(*question).clone()} oninput={on_input} placeholder="Where should I go?" /><button type="submit">{"Ask"}</button></div>
                    </form>
                </aside>
            }) }
        </>
    }
}

fn submit_callback(
    props: &DocentPanelProps,
    question: &UseStateHandle<String>,
    reply: &UseStateHandle<Option<Reply>>,
) -> Callback<SubmitEvent> {
    let catalog = props.catalog.clone();
    let question = question.clone();
    let reply_state = reply.clone();
    let on_answer = props.on_answer.clone();
    Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let next = ask(&catalog, &question);
        reply_state.set(Some(next.clone()));
        on_answer.emit(Some(next));
    })
}

fn reply_view(reply: &Reply) -> Html {
    let heading = match &reply.outcome {
        Outcome::One => "I found a place.",
        Outcome::Several => "A few places fit.",
        Outcome::NotYet(_) => "That place is not open yet.",
        Outcome::Rephrase => "I need a little more detail.",
        Outcome::NothingHere => "I cannot place that here.",
    };
    let detail = match &reply.outcome {
        Outcome::NotYet(message) => message.clone(),
        _ => reply.because.clone(),
    };
    html! { <div class="docent-answer"><strong>{heading}</strong><p>{detail}</p>{ if reply.offer.is_empty() { Html::default() } else { html! { <small>{format!("{} temporary pin(s) placed.", reply.offer.len())}</small> } } }</div> }
}

#[derive(Properties, PartialEq)]
pub struct PinLayerProps {
    pub scene: Scene,
    pub catalog: Catalog,
    pub base: String,
    pub reply: Option<Reply>,
    pub on_navigate: Callback<String>,
}

#[function_component(PinLayer)]
pub fn pin_layer(props: &PinLayerProps) -> Html {
    let Some(reply) = &props.reply else {
        return Html::default();
    };
    html! { <g class="docent-pin-layer">{ for reply.offer.iter().enumerate().filter_map(|(index, id)| pin(id, index, props)) }</g> }
}

fn pin(id: &PlaceId, index: usize, props: &PinLayerProps) -> Option<Html> {
    let hotspot = props
        .scene
        .hotspots
        .iter()
        .find(|hotspot| &hotspot.place == id)?;
    let (x, y) = hotspot.shape.centroid()?;
    let place = props.catalog.get(id)?;
    let path = ancestors(&props.catalog, id)?
        .iter()
        .skip(1)
        .map(|place| place.id.0.clone())
        .collect::<Vec<_>>()
        .join("/");
    let route = path.clone();
    let on_navigate = props.on_navigate.clone();
    let onclick = Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        on_navigate.emit(route.clone());
    });
    Some(html! {
            <a class={classes!("docent-pin", format!("docent-pin-{}", index + 1))} href={format!("{}/campus/{path}", props.base)} aria-label={format!("Docent pin: {}", place.title)} onclick={onclick}>
            <g transform={format!("translate({x} {y})")}><path class="docent-pin-stem" d="M 0 7 L 0 28" /><circle class="docent-pin-shadow" cx="2" cy="29" r="5" /><circle class="docent-pin-head" cx="0" cy="0" r="10" /><circle class="docent-pin-glint" cx="-3" cy="-3" r="3" /></g>
        </a>
    })
}

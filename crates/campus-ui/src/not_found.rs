use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { <section class="not-found"><h1>{"Place not found"}</h1><p>{"That campus path does not resolve to a place."}</p></section> }
}

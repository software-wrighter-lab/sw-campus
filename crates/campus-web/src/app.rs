use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="campus-shell">
            <img class="campus-map" src="assets/campus.webp" alt="Software Wrighter Research Campus map" />
            <Footer />
        </main>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer>
            <span>{"Copyright (c) 2026 Michael A Wright"}</span>
            <a href="LICENSE">{"MIT License"}</a>
            <a href="https://github.com/software-wrighter-lab/sw-campus">
                {"Repository: software-wrighter-lab/sw-campus"}
            </a>
            <span>{"Build Host: "}{env!("BUILD_HOST")}</span>
            <span>{"Build Commit: "}{env!("BUILD_SHA")}</span>
            <span>{"Build Time: "}{env!("BUILD_TIMESTAMP")}</span>
        </footer>
    }
}

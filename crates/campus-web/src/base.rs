pub fn basename() -> String {
    let uri = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.base_uri().ok().flatten())
        .unwrap_or_default();
    let path = uri.split_once("://").map_or(uri.as_str(), |(_, rest)| {
        rest.split_once('/').map_or("", |(_, path)| path)
    });
    let path = format!("/{}", path.trim_matches('/'));
    if path == "/" {
        String::new()
    } else {
        path.trim_end_matches('/').to_owned()
    }
}

use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! { <footer><span>{"Copyright (c) 2026 Michael A Wright"}</span><a href="LICENSE">{"MIT License"}</a><a href="https://github.com/software-wrighter-lab/sw-campus">{"Repository: software-wrighter-lab/sw-campus"}</a><span>{"Build Host: "}{env!("BUILD_HOST")}</span><span>{"Build Commit: "}{env!("BUILD_SHA")}</span><span>{"Build Time: "}{env!("BUILD_TIMESTAMP")}</span></footer> }
}

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

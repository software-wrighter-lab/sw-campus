# Mockups

Static, throwaway HTML mockups used to agree on the flow before it is built
in Rust/Yew/WASM. They are not the app; the app is built by the saga in
`docs/plan.md`. Nothing here is served in production.

## campus/

Campus map -> Computer History Museum lobby -> directory row -> wing landing
page with links to the `sw-comp-history` live demos. Hash-routed so every
state has a URL (`#/campus/computer-history/ibm-1130`). Esc goes up, H goes
to the campus.

Serve it statically from this directory, for example:

```sh
cd mockups/campus && trunk serve --open 2>/dev/null || ruby -run -e httpd . -p 8000
```

or open `mockups/campus/index.html` directly in a browser.

Layout:

```
campus/
|-- index.html        the page and its content tables (PLACES, SCENES)
|-- css/campus.css    the styles (light and dark themes)
`-- images/           web-sized copies of images/*.png at the repo root
```

The `PLACES` and `SCENES` tables in `index.html` are the first drafts of
`content/catalog.ron` and `content/scenes/*.ron`; the hotspot rectangles
match the tables in `docs/plan.md`.

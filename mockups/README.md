# Mockups

Static, throwaway HTML mockups used to agree on the flow before it is built
in Rust/Yew/WASM. They are not the app; the app is built by the saga in
`docs/plan.md`.

## The campus mockup lives in `../pages/`

Campus map -> building lobby -> directory row -> wing landing page with
links to the live demos of `sw-comp-history`, `sw-embed`, and
`sw-ml-study`. Hash-routed so every state has a URL
(`#/campus/computational-sciences/machine-learning`). Esc goes up, H goes
to the campus.

It is published by `.github/workflows/pages.yml` to
<https://software-wrighter-lab.github.io/sw-campus/> on every push to
`main` that touches `pages/`. To serve it locally:

```sh
cd pages && ruby -run -e httpd . -p 8000
```

Layout:

```
pages/
|-- index.html        the page and its content tables (PLACES, SCENES)
|-- css/campus.css    the styles (light and dark themes)
`-- images/           web-sized copies of images/*.png at the repo root
```

The `PLACES` and `SCENES` tables in `index.html` are the first drafts of
`content/catalog.ron` and `content/scenes/*.ron`; the hotspot rectangles
match the tables in `docs/plan.md`. When the Yew app replaces this site,
this directory keeps the mockup for reference.

# Saga: campus-mvp

Build the Software Wrighter Research Campus MVP: a Rust/Yew/WASM single-page
app where a visitor drills down from the painted campus map
(images/sw-campus.png) into the Computer History Museum lobby
(images/museum-lobby.png), into the IBM 1130 wing, and onto individual
exhibits (029, 1130, 1442, ...). Every place has a clean, shareable URL.
Deployed to https://software-wrighter-lab.github.io/sw-campus/ first;
campus.softwarewrighter.com is a later saga.

The full plan, architecture, content tree, hotspot coordinates and hosting
decisions are in docs/plan.md. Read it before every step. docs/research.txt
is background only; where it and docs/plan.md disagree, the plan wins.

Every step ends green: `just check` (fmt, clippy -D warnings, tests, trunk
build, sw-checklist with ZERO failures and ZERO warnings) before commit,
then `agentrail complete`.

## Steps

1. scaffold-workspace -- workspace, campus-web showing the campus image,
   compliant index.html/footer/build.rs, justfile, `just check` green.
2. world-model -- campus-model crate: Place graph types, RON catalog for the
   MVP tree, resolve/ancestors/url, validation tests.
3. scene-engine -- campus-scene crate: Scene/Hotspot types, RON scenes for
   campus + lobby, cross-reference test against the catalog.
4. router-shell -- routes, basename from <base>, breadcrumb, Directory page,
   not-found; every URL in the tree renders.
5. campus-hotspots -- SceneView/HotspotLayer/CaptionCard on the campus;
   hover, click, keyboard; WebP assets; placeholders say "Opening soon".
6. museum-lobby -- lobby scene live with all hotspots; only ibm-1130 open.
7. ibm-1130-wing -- SVG floor-plan scene with 029/1130/1442/1132/2310.
8. exhibit-pages -- ExhibitPage + content for 029, 1130, 1442, radio.
9. pages-deploy -- CI workflow, 404.html, live on github.io, deep link verified.
10. hotspot-editor -- `?edit` overlay traces polygons to RON; refine hotspots.
11. polish-and-close -- keys, focus, mobile, transitions, a11y, close saga.

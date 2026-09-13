# Implementation plan: Software Wrighter Research Campus

Derived from `docs/research.txt` (the design conversation), the two pieces of
art in `images/`, and the MVP as stated by the owner:

> A map where I can drill down from the campus view to the Computer History
> Museum lobby to the IBM 1130 wing to see individual exhibits for 029, 1130,
> 1442, etc.

`docs/research.txt` is input, not specification. Where this plan and the
research disagree, this plan wins; where this plan is silent, the research
is a good default.

## The thesis

Don't build a graphical list of projects. **Build a place where the projects
live.** The information hierarchy is spatial:

```
Campus -> Building -> Wing -> Exhibit -> (Demo)
```

and every node has its own stable URL that can be sent to somebody.

Three rules keep it coherent as it grows (from the research, adopted here):

- **Buildings** are fields of exploration. **Wings** are subjects.
  **Exhibits** are actual work. **Repositories** are implementation
  artifacts attached to exhibits, never the unit of navigation.
- Every place has one canonical home. Other places may *reference* it
  ("Related work"), never duplicate it.
- The semantic model (what exists, what contains what) never knows where
  anything is drawn. Scenes are a separate, replaceable layer.

## What changed from the research: the art already exists

The research proposed drawing isometric buildings as SVG polygons. Since
then the owner produced painted scene art (`images/sw-campus.png`,
`images/museum-lobby.png`, both 1536x1024), and more will arrive "as we go".
So the renderer is not a polygon generator. It is:

> **A scene is a painted image plus a data-driven layer of clickable
> hotspots drawn in SVG on top of it.**

One engine, every level. A scene that has no art yet renders a plain
*directory* (a list of its children) so the URL, breadcrumb and drill-down
all work before the picture arrives. Adding art to a scene is a content
change (one image, one RON file), not a code change.

Consequences:

- SVG `viewBox` is the image's pixel space (`0 0 1536 1024`), so hotspot
  coordinates are stable regardless of viewport, and the whole scene scales
  with `preserveAspectRatio`.
- Hover = highlight the hotspot polygon + show a caption card. Click =
  navigate (real `<a href>` inside the SVG, so open-in-new-tab and middle
  click work). Keyboard: Tab focuses hotspots, Enter enters, Esc goes up.
- The painted-in mock chrome in `museum-lobby.png` (the breadcrumb pill,
  the +/- zoom, the "Map" button) is *reference for the real chrome*, which
  the app renders itself. Until the art is regenerated without it, the
  real breadcrumb overlays the same corner and the fake buttons are simply
  not hotspots.
- The research's isometric projection module is deferred. It becomes
  useful only if we later draw scenes procedurally; nothing in the MVP
  needs it.

## Stack and constraints

- **Rust 2024 edition, Yew 0.21 (CSR), yew-router, Trunk.** No hand-written
  JavaScript beyond what Trunk emits; no Python anywhere in the pipeline.
- **SVG in the DOM, not WebGL.** Tens of objects per scene; SVG gives hit
  testing, focus, ARIA and CSS for free.
- **RON for content** (`ron` + `serde`), embedded with `include_str!` and
  parsed at startup. A native unit test parses every file and validates
  every cross-reference, so content mistakes fail `cargo test`, not the
  visitor.
- **sw-checklist stays at zero failures and zero warnings.** Design to the
  gate (<= 25 LOC per function, <= 4 functions per module, <= 4 modules
  per crate), not to the fail line. This is why the workspace is several
  small crates rather than one.
- **Web UI checks** (`sw-checklist` on the WASM crate): `index.html` in the
  crate directory referencing `favicon.ico`, and a `<footer>` containing
  Copyright, License, Repository, Build Host, Build Commit, Build Time.
  `build.rs` emits the build facts as env vars (pattern from
  `sw-mlpl/components/web/crates/mlpl-web/build_env.rs`).

## Hosting and URLs

Two deployments, same build, different `--public-url`:

| Stage | URL | Trunk `--public-url` |
| --- | --- | --- |
| 1 (this saga) | `https://software-wrighter-lab.github.io/sw-campus/` | `/sw-campus/` |
| 2 (later saga) | `https://campus.softwarewrighter.com/` | `/` + `CNAME` file |

Clean paths from day one (no hash routing):

```
/                                         campus (home)
/campus                                   alias for /
/campus/computer-history                  museum lobby
/campus/computer-history/ibm-1130         wing
/campus/computer-history/ibm-1130/1442    exhibit
/campus/computer-history/ibm-1130/1442/radio   demo under an exhibit
```

How that survives static hosting:

- `index.html` carries `<base data-trunk-public-url />`; Trunk rewrites it
  to `<base href="/sw-campus/">`. The app reads `document.baseURI` at start
  and passes the path as `basename` to `BrowserRouter`. Nothing is
  hard-coded, so the same binary serves both deployments.
- The route table is `/`, `/campus`, `/campus/*path` and a not-found route.
  `*path` is split on `/` and resolved against the catalog. Unlimited depth,
  one code path, no route enum churn when a level is added.
- GitHub Pages deep links: the deploy step copies the built `index.html`
  to `404.html`. Pages serves it (with a 404 status) for any unknown path,
  `<base href>` makes the assets load, and the router reads the real
  pathname. Good enough for humans and shared links. If the 404 status
  ever matters (crawlers), swap in the sessionStorage redirect trick; it
  is contained in the deploy step.
- Stage 2 needs only a `CNAME` file in `dist/` and `--public-url /`.

## Architecture

```
sw-campus/
|-- Cargo.toml                 workspace (edition 2024, clippy pedantic deny)
|-- justfile                   build / serve / check / pages
|-- content/                   RON, the only place content lives
|   |-- catalog.ron            the Place graph (semantic model)
|   `-- scenes/
|       |-- campus.ron         image + hotspots for the campus
|       |-- computer-history.ron
|       `-- ibm-1130.ron
|-- images/                    SOURCE art (PNG originals). Never served.
|-- crates/
|   |-- campus-model/          Place graph. No web deps. Native tests.
|   |-- campus-scene/          Scene + hotspot geometry, RON loading,
|   |                          cross-reference validation. No web deps.
|   |-- campus-ui/             Yew components (scene view, hotspot layer,
|   |                          caption card, breadcrumb, directory,
|   |                          exhibit page, footer)
|   `-- campus-web/            The binary: main, app, route, basename.
|       |-- index.html         Trunk entry, favicon, footer markup
|       |-- Trunk.toml
|       |-- build.rs           BUILD_HOST / BUILD_SHA / BUILD_TIMESTAMP
|       |-- favicon.ico
|       `-- assets/            web-optimized art (WebP), copied by Trunk
|-- docs/
`-- .github/workflows/pages.yml
```

If `campus-ui` reaches five modules, split it into `campus-ui-scene`
(scene view, hotspot layer, caption) and `campus-ui-chrome` (breadcrumb,
directory, exhibit page, footer) *before* adding the fifth. Do not let a
crate reach the fail line.

### Semantic model (`campus-model`)

```rust
pub struct PlaceId(String);            // slug: "ibm-1130"

pub enum PlaceKind { Campus, Building, Wing, Exhibit, Demo, Site }

pub enum Status { Open, Placeholder, ComingSoon }

pub struct Place {
    pub id: PlaceId,
    pub kind: PlaceKind,
    pub title: String,                 // "IBM 1130 Wing"
    pub tagline: String,               // one line, for the caption card
    pub summary: String,               // a paragraph, for the page
    pub status: Status,
    pub children: Vec<PlaceId>,        // ordered, as the directory lists them
    pub scene: Option<String>,         // scene file id, if art exists
    pub links: Vec<Link>,              // Run exhibit / GitHub / Docs / Related
}

pub struct Link { pub label: String, pub url: String, pub kind: LinkKind }
pub enum LinkKind { Run, Source, Docs, Related }
```

`Catalog` owns the places and answers: `root()`, `get(id)`,
`resolve(&[&str]) -> Option<&Place>` (walks children by slug so a path is
valid only if every segment is a child of the previous), `ancestors(id)`
(for the breadcrumb), and `url(id)`. Places never store their own URL; it is
derived from the ancestor chain so a rename never leaves a stale path.

The catalog is one RON file. Validation (a unit test): every child id
exists, no cycles, slugs are `[a-z0-9-]+`, exactly one `Campus`, every
`scene` names a scene file that exists and whose hotspots all point at
*children of that place*.

### Scene model (`campus-scene`)

```rust
pub struct Scene {
    pub id: String,
    pub image: String,                 // "campus.webp", relative to assets/
    pub width: u32, pub height: u32,   // 1536 x 1024
    pub hotspots: Vec<Hotspot>,
}

pub struct Hotspot {
    pub place: PlaceId,                // what it enters
    pub shape: Shape,                  // Polygon(Vec<(f32,f32)>) | Rect{x,y,w,h}
    pub label_at: Option<(f32, f32)>,  // where the caption card anchors
}
```

Rectangles cover most cases (buildings, directory rows). Polygons are for
the irregular ones. Both render as `<polygon>`.

### Web crate

- `main.rs`: mount `App`.
- `app.rs`: `BrowserRouter basename=…`, `Switch`, the page shell
  (header with breadcrumb, main scene or page, footer).
- `route.rs`: the four-variant `Route` and the switch fn that turns
  `*path` into a resolved `Place` (or the not-found page).
- `base.rs`: `basename()` from `document.baseURI`.

Rendering decision per place, in order: if the place has a scene, render
`SceneView` (image + `HotspotLayer`) with the `Directory` beneath it for
accessibility and for children with no hotspot; else if it has children,
render `Directory` alone; else render `ExhibitPage`.

### Interaction

- Hover: hotspot polygon gets a soft fill + stroke; `CaptionCard` appears
  near `label_at` with title, tagline and the first few children's titles
  ("IBM 1130 - System/360 - RCA 1802 ...").
- Click / Enter: `yew_router` `Link` navigation. Back and forward are
  browser history.
- Esc: navigate to the parent. `H`: campus.
- Every hotspot: `role="link"`, `tabindex="0"`, `aria-label`.
- Placeholder places still get a hotspot and a caption, but the caption
  says "Opening soon" and the click lands on the directory page for that
  place, which says the same. A campus that visibly can grow beats a
  campus that pretends to be finished.

## Content for the MVP

Only the vertical slice is *open*. Everything else on the two paintings is
present as a placeholder so hover and URLs work everywhere.

```
campus                                   Software Wrighter Research Campus   [scene: campus]
|-- computer-history                     Computer History Museum             [scene: computer-history]
|   |-- ibm-1130                         IBM 1130 Wing                       [scene: ibm-1130]
|   |   |-- 029                          IBM 029 Card Punch                  open
|   |   |-- 1130                         IBM 1130 Console (1131 CPU)         open
|   |   |-- 1442                         IBM 1442 Card Read Punch            open
|   |   |   `-- radio                    Card-reader radio: music by RFI     open
|   |   |-- 1132                         IBM 1132 Printer                    placeholder
|   |   `-- 2310                         IBM 2310 Disk Drive                 placeholder
|   |-- ibm-360, ibm-370, ibm-390        wings                               placeholder
|   |-- early-computers, personal-computing, processors, interfaces-media,
|   |   minicomputers, game-machines, special-exhibits, library-archives     placeholder
|   `-- lobby floor: vacuum-tubes, transistors, punch-cards, core-memory,
|       paper-tape, early-pcs            lobby exhibits                      placeholder
|-- computer-science                     Computer Science Building           placeholder
|-- hardware-lab                         Hardware Lab                        placeholder
|-- computational-sciences               Computational Sciences Institute    placeholder
|-- digital-media                        Digital Media Studio                placeholder
|-- interactive-computing                Interactive Computing Lab           placeholder
|-- commons                              The Commons                         placeholder (about page)
`-- future-nw, future-ne, future-sw      Future Building                     coming-soon
```

Wing lists for the placeholder buildings come from the captions painted on
`sw-campus.png` (Computer Science: Architecture, Operating Systems,
Programming Languages, Compilers, COR24, RISC-V; Hardware Lab: FPGA, SoCs,
MCUs, Prototyping, Test & Measurement; and so on). They are the *tagline*
text for now, not children; children are added when a building gets its
own saga.

Known external targets for the open exhibits (fill in as each step lands):

- IBM 1130 system simulator: `https://github.com/softwarewrighter/demo-ibm-1130-system`,
  live at `https://softwarewrighter.github.io/demo-ibm-1130-system/`. It
  simulates the 1442, 2310/2311, 1403 and 1133; the 1130 and 1442 exhibits'
  "Run exhibit" links point into it.
- 029 keypunch and the 1442 radio demo: locate the repos when writing the
  exhibit content (not found under the known orgs on 2026-09-12). Until
  then the exhibit page renders with `status: Placeholder` and no Run link.

### First-pass hotspot rectangles

Image space, 1536x1024, eyeballed from the art. Good enough to ship; the
hotspot editor (step 10) refines them into polygons.

Campus (`scenes/campus.ron`):

| place | x | y | w | h |
| --- | --- | --- | --- | --- |
| computer-history | 575 | 130 | 385 | 210 |
| computer-science | 285 | 250 | 280 | 230 |
| hardware-lab | 950 | 250 | 350 | 220 |
| computational-sciences | 230 | 480 | 390 | 260 |
| digital-media | 620 | 600 | 290 | 230 |
| interactive-computing | 1050 | 460 | 390 | 250 |
| commons | 700 | 400 | 180 | 130 |
| future-nw | 60 | 210 | 270 | 150 |
| future-ne | 1290 | 230 | 230 | 140 |
| future-sw | 20 | 760 | 310 | 150 |

Museum lobby (`scenes/computer-history.ron`). The directory board spans
x 575-975, y 375-548 with 12 rows of about 29 px, two columns:

| place | x | y | w | h |
| --- | --- | --- | --- | --- |
| ibm-1130 | 577 | 375 | 193 | 24 |
| ibm-360 | 577 | 404 | 193 | 24 |
| ibm-370 | 577 | 433 | 193 | 24 |
| ibm-390 | 577 | 462 | 193 | 24 |
| early-computers | 577 | 492 | 193 | 24 |
| personal-computing | 577 | 522 | 193 | 24 |
| processors | 792 | 375 | 183 | 24 |
| interfaces-media | 792 | 404 | 183 | 24 |
| minicomputers | 792 | 433 | 183 | 24 |
| game-machines | 792 | 462 | 183 | 24 |
| special-exhibits | 792 | 492 | 183 | 24 |
| library-archives | 792 | 522 | 183 | 24 |
| vacuum-tubes | 30 | 700 | 205 | 200 |
| transistors | 295 | 720 | 195 | 180 |
| punch-cards | 300 | 610 | 160 | 110 |
| core-memory | 460 | 640 | 140 | 150 |
| paper-tape | 1000 | 640 | 160 | 150 |
| early-pcs | 1170 | 690 | 200 | 160 |

The wall banners (left: "IBM 1130 / System/360 / 370 / 390") can be a
second hotspot for `ibm-1130` later; one hotspot per place is enough now.

### Two more lobbies (added 2026-09-12, afternoon)

Two more paintings arrived and settled the "where does ML live" question
by organisation: the Computer Science Building's board is the **sw-embed
Project Directory**, the Computational Sciences Institute's board is
**Explore the sw-ml-study Repositories**. The Computer History Museum is
sw-comp-history. One org per building, so far.

Computer Science Building (`images/computer-science-lobby.png`, board
x 522-965, rows 40 px tall starting y 213 with a 44 px pitch):

| row | place | repos behind it |
| --- | --- | --- |
| 1 | cpu-architectures | cor24-rs, risc-v-rs, web-sw-cor24-demos, sw-cor24-isa, hw-cor24-tang-nano |
| 2 | language-experiments | 12 live web-sw-cor24-* language demos + their toolchain repos |
| 3 | operating-systems | swtos-live, web-sw-tos, sw-tos; sw-ml-study/sw-os-ml cross-listed; MesaOS is an external fork |
| 4 | emulators-simulators | the browser IDEs/debuggers, same demos as rows 1 and 2 seen as "things you can Run" |
| 5 | hardware-interfaces | placeholder: bmp280, hardwarewrighter repos; no live demo |
| 6 | development-tools | x-assembler, x-tinyc, pcode (live); monitor, script, yocto-ed, debugger, aotc |
| 7 | demos-experiments | web-sw-cor24-demos |
| 8 | all-sw-embed | outbound link to github.com/sw-embed |

Floor: cpus-through-ages (x 0-410, y 640-830; the Z80 and 6502 in the case
have no repos), interfaces-peripherals (1150-1536, 640-840),
information-desk (500-1080, 620-730), featured-projects (1355-1536, 405-600).

Computational Sciences Institute (`images/computational-sciences-institute-lobby.png`,
second version; board x 588-888, rows 38 px starting y 308 with a 45 px pitch):

| row | place | repos behind it |
| --- | --- | --- |
| 1 | sw-mlpl | sw-mlpl (latest), mlpl-live (stable at mlpl.softwarewrighter.com), demo-mlpl-libraries |
| 2 | mathematics-foundation | demo-abstract-algebra (site, course, lab, all live), demo-linear-algebra; calculus and discrete are empty |
| 3 | category-theory | demo-category-theory (no browser build) |
| 4 | machine-learning | demo-ml-utils, demo-ml-microscope, moe-microscope, emufpga (live), neural-net-rs (live), ml-viz, cat-finder | The MoE Microscope is its own exhibit (early); it becomes the Institute's featured exhibit once moe-microscope publishes its browser page. |
| 5 | algorithms-data-structures | demo-algorithms, demo-data-structures, demo-memory |
| 6 | functional-pipelines | demo-functional-pipelines, demo-combinators, demo-design-patterns |
| 7 | extensions-integrations | demo-extensions, demo-file-processing, demo-mlpl-libraries |
| 8 | all-sw-ml-study | outbound link to github.com/sw-ml-study |

Floor: abstraction-to-intelligence (0-370, 700-860), curiosity-fountain
(610-980, 660-790), visualizing-understanding (1180-1536, 700-870),
ideas-lab (1140-1330, 530-690).

The second version of this painting fixed the two naming problems the
first one had (it now says "Institute" everywhere, and the ML row is
"Machine Learning", not "ML Utilities").

Known mismatches to resolve when the art is regenerated (queue: wing-art):

- Both paintings carry mock chrome (campus sign, breadcrumb pill).
- Nine sw-ml-study repos are forks of other people's work (ATTN-11, bertviz,
  PredictiveCoding, Engram, Diffusion, JEPA, ViT, Repeated-Sampling,
  Bonsai-Image-Demo). They are not exhibits; at most a "reading shelf".

IBM 1130 wing: no art yet. Step 7 draws a simple SVG floor plan (console
centre, 1442 left, 1132 right, keypunch area front, disk at the back) as the
scene image, so the wing works end to end and the painted art replaces it
by changing one filename.

## Build, run, check

```
just serve          trunk serve (hot reload) at http://127.0.0.1:8080/sw-campus/
just build          trunk build --release --public-url /sw-campus/  -> dist/
just check          cargo fmt --check, clippy -D warnings (host + wasm32),
                    cargo test, trunk build, sw-checklist
just pages          what CI runs: build + copy 404.html + tree of dist/
```

`just check` is the pre-commit gate. It must pass before `agentrail
complete`, every step, no exceptions.

CI (`.github/workflows/pages.yml`): on push to `main`, install the stable
toolchain with `wasm32-unknown-unknown`, install Trunk from its release
binary, `just pages`, upload `dist/` with `actions/upload-pages-artifact`,
deploy with `actions/deploy-pages`. Modelled on
`sw-ml-study/demo-abstract-algebra/.github/workflows/pages.yml`.

## Saga: campus-mvp

One saga, eleven steps, each a single session that ends green. Steps 2 and
3 have no web dependency and are pure TDD. Nothing in steps 1-4 needs art;
the vertical slice becomes visible at step 5.

| # | slug | delivers |
| --- | --- | --- |
| 1 | scaffold-workspace | Workspace, `campus-web` showing the campus image full-bleed, index.html with favicon + compliant footer, `build.rs`, justfile, `just check` green, sw-checklist 0/0 |
| 2 | world-model | `campus-model`: types, RON catalog for the MVP tree, `resolve`/`ancestors`/`url`, validation tests |
| 3 | scene-engine | `campus-scene`: Scene/Hotspot types, RON scenes for campus + lobby using the tables above, cross-reference test against the catalog |
| 4 | router-shell | Routes, basename from `<base>`, breadcrumb, `Directory` page for any place, not-found page; every URL in the tree renders something |
| 5 | campus-hotspots | `SceneView` + `HotspotLayer` + `CaptionCard` on the campus; hover/click/keyboard; WebP assets; placeholders show "Opening soon" |
| 6 | museum-lobby | Lobby scene live with all 18 hotspots; only `ibm-1130` is open; the rest land on their directory page |
| 7 | ibm-1130-wing | SVG floor-plan scene for the wing with 029 / 1130 / 1442 / 1132 / 2310 hotspots |
| 8 | exhibit-pages | `ExhibitPage`: title, summary, Run / Source / Docs / Related links; content for 029, 1130, 1442, radio |
| 9 | pages-deploy | CI workflow, `404.html`, live at the github.io URL; README links; deep link verified from a fresh browser |
| 10 | hotspot-editor | `?edit` overlay (Rust only): click to trace a polygon, prints the RON hotspot to the console; refine campus + lobby polygons with it |
| 11 | polish-and-close | Esc/H keys, focus rings, mobile width, transition fade between scenes, a11y pass, `sw-checklist` 0/0, close the saga |

## Saga: campus-docent (after campus-mvp step 4)

Source: `../../sw-ml-study/moe-microscope/docs/research/research3.txt` and
that repo's work order in `docs/implementation/cross-repo-handoffs.md`.
The docent is a self-guided audio tour delivered as a chat window. A tiny
mixture-of-experts model, trained in batch in moe-microscope, predicts only
an intent (navigate, explain, recommend, story, unsupported) and a
destination; this catalog supplies every title, URL, breadcrumb, status,
and every word of story text. Stories are canned on the Place. The model
never writes prose, so a stale model can never invent an exhibit.

The mockup in `pages/` already has the whole tour metaphor with a keyword
matcher standing in for the model (added 2026-09-13): an easel in the same
corner of every scene with a featured exhibit and "Ask the docent"; a
drawer with stop cards (stop number, place, story, Take me there, Another
story, Play, Why this?); arrival stories volunteered once; told stories,
recent places, interests and recent queries kept in local storage; Clear
my tour; the edition line naming the snapshot the stories came from.
`pages/docent/snapshot-a.json` is moe-microscope's snapshot A verbatim.

Steps, each one session, all `sw-checklist`-clean:

The docent also speaks for the work in progress. Every place has a
maturity: finished (complete and stable), working (runs today, still
improving), early (source exists, nothing runnable), or planned (on the
map, nothing built). Maturity is derived from the catalog (a live demo or
scene means working, repos only means early, placeholder means planned)
unless the place carries a hand-written `progress` note, which overrides
it and adds one or two sentences. The "status" intent ("what works?", "is
the ML wing finished?") answers from the four pre-written maturity
sentences plus the note plus a tally of the children; nothing is
generated. Directory chips show the same four labels, and arriving at a
non-working place gets the status line instead of a story.

1. **docent-block-and-export.** `Docent { aliases, concepts, example_queries,
   stories: [Story { id, title, text, concepts, kind }] }` as an optional
   field on Place; `maturity: Option<Maturity>` and `progress: Option<String>`
   on Place, with `maturity_of()` deriving the default; `featured: Option<PlaceId>` on lobbies; the three exhibit
   places snapshot A expects (ibm-1130-emulator, rca-1802, apl); a test that
   writes `dist/catalog.json` canonically with its SHA-256 so moe-microscope's
   corpus generator reads the live catalog and the manifest can name it.
2. **easel.** `campus-docent` crate: `Easel` component (passive face: featured
   exhibit, Visit; interactive face opens the drawer) rendered by SceneView
   and by Directory/Exhibit pages; same corner on every scene.
3. **tour-drawer.** `Docent` drawer with the transcript of stop cards, chips,
   input, Clear my tour; `DocentContext` in IndexedDB (current place, recent
   places, interests, recent queries, told stories) and open/closed state in
   session storage; the story policy ported from moe-microscope with its
   scripted-visit tests; Play via web-sys SpeechSynthesis; Esc closes first.
4. **catalog-matcher.** The deterministic fallback `Predictor` (alias and
   concept matching, the mockup's algorithm) behind a trait, so the easel
   ships and works before any weights exist; "Why this?" shows its signals.
5. **model-bridge.** The `mlpl-wasm` `Predictor`: load `docent/model`,
   `labels`, `manifest` at startup; compare the manifest's catalog hash with
   the live one and show the edition line and stale badge; run inference in
   the same session moe-microscope's page uses (iframe bridge now, headless
   build when upstream ships it). "Why this?" shows the router's experts.
6. **snapshot-b.** When the 1442 and its radio demo are published: export
   snapshot B, hand it to moe-microscope for their v1 saga, and show the
   stale badge for real until the retrained weights land.

Steps 1 to 4 can start as soon as campus-mvp step 4 (router-shell) exists.
Step 5 waits on moe-microscope step 4 (`just docent` batch export) and their
step 5 (measured browser inference through the bridge).

## Saga queue (after campus-mvp)

Not started, not scheduled, listed so they are not forgotten:

- **custom-domain** -- `campus.softwarewrighter.com`: CNAME, `--public-url /`,
  DNS, HTTPS, redirect from the github.io URL.
- **wing-art** -- replace the SVG floor plan with painted 1130-wing art;
  lobby art regenerated without mock chrome; hotspot polygons retraced.
- **museum-wings** -- System/360, /370, /390, RCA 1802 wings and the lobby
  floor exhibits get real content.
- **computer-science** -- the CS building lobby; COR24 as a department with
  its many `sw-embed` demos as exhibits; RISC-V; sw-TOS.
- **cross-campus-paths** -- "Related work" links rendered as paths between
  places (the research's strongest idea, deliberately deferred).
- **scene-editor** -- drag hotspots, export RON; grows out of step 10.

## Non-goals for the MVP

- No WebGL, no Three.js, no continuous zoom from campus to room.
- No search, no project index page. The map is the index.
- No content management beyond RON in git.
- No isometric projection module until a scene is drawn procedurally.

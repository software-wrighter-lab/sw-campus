# sw-campus -- Software Wrighter Research Campus

**[Open the campus](https://software-wrighter-lab.github.io/sw-campus/)** --
explore the live Rust/Yew/WASM campus map, enter a building, and follow its
directories to wings and exhibits. The idea is introduced in the blog post
[Software Wrighter Research Campus](https://blog.softwarewrighter.com/2026/09/12/software-wrighter-research-campus/).

A visual, explorable guide to the public projects and live demos of
Software Wrighter, drawn as a research campus you can walk into.

Instead of a list of repositories, the front door is a painted campus map.
Hover a building to see what it holds; click it to enter. Inside, a museum
lobby has a directory; a wing has exhibits; an exhibit has the demo, the
story, and the source. Every place has its own URL, so a link can point at
the whole campus, one building, one wing, or one exhibit.

```
Campus -> Building -> Wing -> Exhibit -> Demo
```

Built in Rust with Yew and WebAssembly, rendered as SVG over painted scene
art. No JavaScript beyond what the bundler emits, no Python.

## Status

The first vertical slice is live: campus map, Computer History Museum lobby,
IBM 1130 wing, and individual exhibit pages. See [docs/plan.md](docs/plan.md)
for the architecture and `agentrail status` for saga progress.

The MVP is one complete vertical slice: campus map, Computer History
Museum lobby, IBM 1130 wing, and exhibits for the 029 card punch, the 1130
console, and the 1442 card read punch.

It is published at <https://software-wrighter-lab.github.io/sw-campus/> and
will later also appear at `https://campus.softwarewrighter.com/`.

## Purpose

- **Explain, not enumerate.** Buildings are fields of exploration, wings
  are subjects, exhibits are actual work. Repositories are attached to
  exhibits; visitors never have to understand repo organisation first.
- **Be sendable.** Any place is a clean, stable URL.
- **Grow organically.** Placeholder buildings and "future building" sites
  are part of the map on purpose. A campus that can visibly grow is more
  convincing than one packed with everything at once.

## Documents

| Document | What it answers |
| --- | --- |
| [docs/plan.md](docs/plan.md) | Architecture, content model, URLs, hosting, and the implementation saga |
| docs/research.txt | Raw design conversation the plan was distilled from |
| [CLAUDE.md](CLAUDE.md) | Rules for coding agents working this repo |
| [mockups/README.md](mockups/README.md) | The static mockup in `pages/` and how to serve it locally |
| [Blog post](https://blog.softwarewrighter.com/2026/09/12/software-wrighter-research-campus/) | Why the campus exists, for visitors |

Source art lives in `images/` (`sw-campus.png`, `museum-lobby.png`). Those
are the originals; the web build serves optimised copies.

## Building

Prerequisites: a stable Rust toolchain with the `wasm32-unknown-unknown`
target, [Trunk](https://trunkrs.dev), [just](https://just.systems), and
`sw-checklist` (Software Wrighter's conformance checker, on the PATH).

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk just
```

Then:

```sh
just serve     # hot-reloading dev server at http://127.0.0.1:8080/sw-campus/
just build     # release build into dist/
just check     # the pre-commit gate: fmt, clippy, tests, trunk build, sw-checklist
just pages     # exactly what CI publishes to GitHub Pages
```

(These recipes land with the first saga step; until then this section is
the contract they must satisfy.)

## Development

Work is driven by [agentrail](CLAUDE.md) sagas. Start a session with
`agentrail next`, do the step, run `just check`, commit, then
`agentrail complete`. Code metrics are enforced by `sw-checklist`, which
must report zero failures and zero warnings at every commit.

For geometry work, add ?edit to a scene URL while running just serve.
Click vertices on the painting, press Enter to log a RON polygon, Backspace
to undo a vertex, or Esc to clear the draft. This tracing overlay is
development-only and is absent from normal URLs.

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).

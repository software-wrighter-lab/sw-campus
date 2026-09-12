Scaffold the Rust/Yew/WASM workspace so `just serve` shows the campus painting full-bleed and `just check` is green.

Read docs/plan.md ("Stack and constraints", "Hosting and URLs", "Architecture", "Build, run, check") first.

Deliver:
- Root Cargo.toml workspace: members crates/campus-web only for now; edition = "2024" via [workspace.package]; [workspace.lints.clippy] all = "deny", pedantic = "deny"; rust-version 1.85.
- crates/campus-web: bin crate, yew 0.21 (csr), yew-router 0.18, web-sys, wasm-bindgen. src/main.rs mounts App; src/app.rs renders a full-bleed <img> (or <svg><image>) of the campus art plus the footer. Keep every fn <= 25 LOC, <= 4 fns per module, <= 4 modules per crate.
- crates/campus-web/index.html: <base data-trunk-public-url />, <link data-trunk rel="rust" data-bin="campus-web" />, <link data-trunk rel="icon" href="favicon.ico" />, <link data-trunk rel="copy-dir" href="assets" />, a <footer> with the literal text Copyright (c) 2026 Michael A Wright, a link "MIT License" to LICENSE, a link "Repository: software-wrighter-lab/sw-campus", and Build Host / Build Commit / Build Time (values rendered by a Footer component from env!("BUILD_HOST"), env!("BUILD_SHA"), env!("BUILD_TIMESTAMP")). Confirm with `sw-checklist -v crates/campus-web` that Web UI, index.html, Favicon Reference, favicon.ico, Footer Presence, Copyright, License, Repository, Build Host, Build Commit, Build Time all PASS.
- crates/campus-web/build.rs + build_env.rs emitting BUILD_HOST, BUILD_SHA, BUILD_TIMESTAMP (pattern: ~/github/sw-ml-study/sw-mlpl/components/web/crates/mlpl-web/build_env.rs).
- crates/campus-web/favicon.ico (a simple generated icon is fine; no Python, use an SVG->ICO via a Rust tool or hand-craft a 16x16 ICO).
- crates/campus-web/assets/campus.webp: images/sw-campus.png converted to WebP (target < 400 KB; `sips` or `cwebp` locally, no Python). images/ stays the untouched source.
- crates/campus-web/Trunk.toml with dist = "../../dist".
- justfile at the root with recipes serve, build, check, pages exactly as docs/plan.md describes them. `check` = cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo clippy --target wasm32-unknown-unknown -p campus-web -- -D warnings, cargo test, trunk build (release, --public-url /sw-campus/), sw-checklist. `pages` = build then cp dist/index.html dist/404.html.
- .gitignore already covers /target/ and /dist/.

Verify: `just serve` shows the painting; `just check` passes; `sw-checklist` prints 0 failed, 0 warnings. Commit, then `agentrail complete`.

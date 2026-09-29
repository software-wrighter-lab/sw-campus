set shell := ["zsh", "-cu"]

serve:
    NO_COLOR=false trunk serve --config crates/campus-web/Trunk.toml --dist ../../dist --public-url /sw-campus/

build:
    NO_COLOR=false trunk build --config crates/campus-web/Trunk.toml --dist ../../dist --release --public-url /sw-campus/

check:
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings
    cargo clippy --target wasm32-unknown-unknown -p campus-web -- -D warnings
    cargo test
    NO_COLOR=false trunk build --config crates/campus-web/Trunk.toml --dist ../../dist --release --public-url /sw-campus/
    sw-checklist -v crates/campus-web

pages: build
    cp dist/index.html dist/404.html

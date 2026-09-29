mod app;
mod base;
mod route;

fn main() {
    yew::Renderer::<app::App>::new().render();
}

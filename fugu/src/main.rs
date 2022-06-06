mod app;
mod components;
mod theme;
mod containers;

#[tokio::main]
async fn main() {
    dioxus::desktop::launch(app::app);
}

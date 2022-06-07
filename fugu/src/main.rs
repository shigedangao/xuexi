mod app;
mod components;
mod theme;
mod containers;
mod state;

#[tokio::main]
async fn main() {
    dioxus::desktop::launch(app::app);
}

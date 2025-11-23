// Application entry point for the Yew app
use yew::Renderer;
mod app;
mod components; // Declare top-level modules here
mod data;       // Declare top-level modules here

fn main() {
    // Initializes the Yew application and renders the root component (App)
    Renderer::<app::App>::new().render();
}

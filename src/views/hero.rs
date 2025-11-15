use yew::prelude::*;

/// The main Hero section.
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div class="hero min-h-screen bg-base-200 flex items-center justify-center">
            // Terminal is now rendered by app.rs
            // This component can contain other hero-related content if needed
        </div>
    }
}
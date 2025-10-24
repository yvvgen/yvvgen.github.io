use crate::components::{FontSelector, Terminal, ThemeSwitcher};
use yew::prelude::*;

/// The main Hero section, which serves as the wrapper for the interactive terminal.
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section
            class="min-h-screen flex flex-col items-center justify-center px-4 bg-base-100 relative"
        >
            // --- UI Controls (Fixed) ---
            <div class="absolute top-4 left-4 z-10">
                <FontSelector />
            </div>
            <div class="absolute top-4 right-4 z-10">
                <ThemeSwitcher />
            </div>
            // --- Terminal Container ---
            <div class="mockup-code">
                <Terminal />
            </div>
        </section>
    }
}

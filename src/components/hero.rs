use crate::components::{FontSelector, ThemeSwitcher};
use yew::prelude::*;

/// The main Hero section, which serves as the wrapper for the interactive terminal.
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="min-h-screen flex flex-col items-center justify-center px-4 bg-base-100 relative">
            // --- UI Controls (Fixed) ---
            <div class="absolute top-4 left-4 z-10">
                <FontSelector />
            </div>

            <div class="absolute top-4 right-4 z-10">
                <ThemeSwitcher />
            </div>

            // --- Terminal Container ---
            // The container provides responsive height/width limits for the terminal box.
            <div class="w-full max-w-5xl h-[80vh] sm:h-[75vh] md:h-[70vh] lg:h-[80vh] xl:h-[85vh] flex items-center justify-center">
            </div>
        </section>
    }
}

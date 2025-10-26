use crate::components::terminal::TerminalEmulator;
use yew::prelude::*;

/// The main Hero section, which serves as the wrapper for the interactive terminal.
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div class="hero min-h-screen bg-base-200 flex items-center justify-center">
            <TerminalEmulator />
        </div>
    }
}

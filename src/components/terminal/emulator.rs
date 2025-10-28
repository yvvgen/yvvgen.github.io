use super::TerminalScreen;
use crate::commands::create_command_registry;
use crate::data::terminal::TerminalState;
use std::rc::Rc;
use yew::prelude::*;

#[function_component(TerminalEmulator)]
pub fn terminal_emulator() -> Html {
    // Initialize the command registry once (only on first render)
    let registry = use_state(|| Rc::new(create_command_registry()));

    // Initialize terminal state
    let state = use_state(|| TerminalState {
        current_view: None,
        history: vec![],
        command_buffer: String::new(),
        registry: (*registry).clone(),
    });

    html! {
        <div
            class="w-2xl min-h-80 bg-base-100 rounded-2xl shadow-[0_0_50px_10px_hsla(248,81.1%,78.4%,0.7)] overflow-hidden flex flex-col relative"
        >
            // Main terminal screen
            <TerminalScreen state={state.clone()} />
        </div>
    }
}

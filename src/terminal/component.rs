use super::{screen::TerminalScreen, state::TerminalState, BootState, CommandOutput};
use crate::terminal::commands::create_command_registry;
use yew::prelude::*;
use yew::platform::time::sleep;
use std::rc::Rc;
use std::time::Duration;

#[derive(Properties, PartialEq)]
pub struct TerminalProps {
    pub on_command_complete: Callback<CommandOutput>,
}

#[function_component(Terminal)]
pub fn terminal(props: &TerminalProps) -> Html {
    let boot_state = use_state(|| BootState::Booting);
    let boot_messages = use_state(|| Vec::<String>::new());

    // Simulate booting process
    {
        let boot_state = boot_state.clone();
        let boot_messages = boot_messages.clone();
        use_effect_with(
            (), // Empty deps to run once on mount
            move |_| {
                if *boot_state == BootState::Booting {
                    let boot_messages = boot_messages.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let messages = vec![
                            "Initializing Gemini OS...",
                            "Loading modules...",
                            "Establishing secure connection...",
                            "Boot sequence complete. Type 'help' to begin.",
                        ];
                        for msg in messages {
                            boot_messages.set(vec![(*boot_messages).clone(), vec![msg.to_string()]].concat());
                            sleep(Duration::from_millis(500)).await;
                        }
                        sleep(Duration::from_millis(1000)).await; // Small pause after messages
                        boot_state.set(BootState::Ready);
                    });
                }
                || ()
            },
        );
    }

    // Initialize the command registry once (only on first render)
    let registry = use_state(|| Rc::new(create_command_registry()));

    // Initialize terminal state
    let state = use_state(|| TerminalState {
        current_view: None,
        history: vec![],
        command_buffer: String::new(),
        registry: (*registry).clone(),
    });

    // Pass the on_command_complete callback to the TerminalScreen
    let on_command_complete_for_screen = props.on_command_complete.clone();

    html! {
        <div
            class="w-2xl min-h-80 bg-base-100 rounded-2xl shadow-[0_0_50px_10px_hsla(248,81.1%,78.4%,0.7)] overflow-hidden flex flex-col relative"
        >
            if *boot_state == BootState::Booting {
                <div class="grow p-6 overflow-y-auto text-sm bg-base-100 scroll-smooth font-mono text-success">
                    {
                        (*boot_messages).iter().map(|msg| html!{ <div>{msg}</div> }).collect::<Html>()
                    }
                </div>
            } else {
                <TerminalScreen state={state.clone()} on_command_complete={on_command_complete_for_screen} />
            }
        </div>
    }
}

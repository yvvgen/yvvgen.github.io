use super::{CommandOutput, state::{HistoryEntry, TerminalState}};
use crate::terminal::command_result::CommandResult;

use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalScreenProps {
    pub state: UseStateHandle<TerminalState>,
    pub on_command_complete: Callback<CommandOutput>,
}

#[function_component(TerminalScreen)]
pub fn terminal_screen(props: &TerminalScreenProps) -> Html {
    let state = props.state.clone();
    let input_ref = use_node_ref();
    let container_ref = use_node_ref();

    // Focus input on mount and when view changes
    {
        let input_ref = input_ref.clone();
        let current_view = state.current_view.clone();

        use_effect_with(current_view, move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    // Auto-scroll to bottom when history changes
    {
        let container_ref = container_ref.clone();
        let history_len = state.history.len();

        use_effect_with(history_len, move |_| {
            if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
                container.set_scroll_top(container.scroll_height());
            }
            || ()
        });
    }

    let on_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let mut new_state = (*state).clone();
                new_state.command_buffer = input.value();
                state.set(new_state);
            }
        })
    };

    let on_keydown = {
        let state = state.clone();
        let input_ref = input_ref.clone();
        let on_command_complete = props.on_command_complete.clone();

        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Enter" => {
                    e.prevent_default();

                    let mut new_state = (*state).clone();
                    let cmd = new_state.command_buffer.trim().to_string();

                    if !cmd.is_empty() {
                        // Special handling for "exit" when in a view
                        if (cmd == "exit" || cmd == "back" || cmd == "q")
                            && new_state.current_view.is_some()
                        {
                            new_state.current_view = None;
                            new_state.history.push(HistoryEntry {
                                command: cmd.clone(),
                                output: vec!["Returned to terminal.".to_string()],
                            });
                            on_command_complete.emit(CommandOutput::None);
                        } else {
                            // Execute command through registry
                            let result = new_state.registry.execute(&cmd);

                            match result {
                                CommandResult::Output(output) => {
                                    new_state.history.push(HistoryEntry {
                                        command: cmd.clone(),
                                        output: output.clone(),
                                    });
                                    on_command_complete.emit(CommandOutput::Text(output.join("\n"))); // Emit text output
                                }
                                CommandResult::ChangeView(view) => {
                                    new_state.current_view = Some(view);
                                    on_command_complete.emit(CommandOutput::None); // No text output for view change
                                }
                                CommandResult::Clear => {
                                    new_state.history.clear();
                                    on_command_complete.emit(CommandOutput::None);
                                }
                                CommandResult::Error(msg) => {
                                    new_state.history.push(HistoryEntry {
                                        command: cmd.clone(),
                                        output: vec![format!("Error: {}", msg)],
                                    });
                                    on_command_complete.emit(CommandOutput::Text(format!("Error: {}", msg))); // Emit error output
                                }
                            }
                        }
                    }

                    // Clear command buffer
                    new_state.command_buffer.clear();
                    state.set(new_state);

                    // Clear and refocus input
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        input.set_value("");
                        let _ = input.focus();
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    // TODO: Navigate to previous command in history
                }
                "ArrowDown" => {
                    e.prevent_default();
                    // TODO: Navigate to next command in history
                }
                "Tab" => {
                    e.prevent_default();
                    // TODO: Auto-complete command
                }
                _ => {}
            }
        })
    };

    html! {
        <div
            ref={container_ref}
            class="grow p-6 overflow-y-auto text-sm bg-base-100 scroll-smooth"
        >
            { if let Some(view) = &state.current_view {
                    // Render full-screen view
                    view.render(state.clone())
                } else {
                    // Render normal terminal with history and input
                    html! {
                        <>
                            // Welcome message on first load
                            {
                                if state.history.is_empty() {
                                    html! {
                                        <div class="mb-6 text-base-content/80">
                                            <div class="text-accent mb-2">
                                                { "Welcome to the interactive terminal!" }
                                            </div>
                                            <div class="text-sm">
                                                { "Type " }
                                                <span class="text-primary">{ "help" }</span>
                                                { " to see available commands." }
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }

                            // Render command history
                            {
                                state.history.iter().map(|entry| {
                                    render_history_entry(entry)
                                }).collect::<Html>()
                            }

                            // Command input
                            <div class="flex items-center">
                                <span class="text-success mr-2 select-none">{ "$ " }</span>
                                <input
                                    ref={input_ref}
                                    type="text"
                                    class="grow bg-transparent text-primary focus:outline-none caret-primary placeholder-base-content/40"
                                    placeholder="Enter command..."
                                    value={state.command_buffer.clone()}
                                    oninput={on_input}
                                    onkeydown={on_keydown}
                                    autocomplete="off"
                                    spellcheck="false"
                                />
                            </div>
                        </>
                    }
                } }
        </div>
    }
}

/// Render a single history entry with command and output
fn render_history_entry(entry: &HistoryEntry) -> Html {
    html! {
        <div class="mb-4">
            // Command line with timestamp
            <div class="mb-1 flex items-baseline">
                <span class="text-success mr-2 select-none">{ "$ " }</span>
                <span class="text-primary">{ &entry.command }</span>
            </div>
            // Command output
            { if !entry.output.is_empty() {
                    html! {
                        <div class="text-base-content pl-4 space-y-1">
                            {
                                entry.output.iter().map(|line| {
                                    // Check if line starts with "Error:" for styling
                                    let is_error = line.starts_with("Error:");
                                    let class = if is_error {
                                        "text-error"
                                    } else {
                                        "text-base-content/90"
                                    };

                                    html! {
                                        <div class={class}>{ line }</div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    }
                } else {
                    html! {}
                } }
        </div>
    }
}
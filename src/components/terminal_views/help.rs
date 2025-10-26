use crate::data::terminal::TerminalState;
use crate::utils::terminal::TerminalView;
use yew::prelude::*;

#[derive(Clone)]
pub struct HelpView;

impl TerminalView for HelpView {
    fn id(&self) -> &str {
        "help"
    }

    fn clone_box(&self) -> Box<dyn TerminalView> {
        Box::new(self.clone())
    }

    fn render(&self, state: UseStateHandle<TerminalState>) -> Html {
        let back_to_terminal = {
            let state = state.clone();
            Callback::from(move |_| {
                let mut new_state = (*state).clone();
                new_state.current_view = None;
                state.set(new_state);
            })
        };

        let commands = state.registry.list_commands();

        html! {
            <div class="space-y-6 animate-fade-in">
                // Header
                <div class="border-b border-primary/30 pb-4">
                    <div class="text-accent text-2xl font-bold mb-2">{ "Available Commands" }</div>
                    <div class="text-base-content/60 text-sm">
                        { "Type any command below to execute it. Use 'exit' to return to the terminal." }
                    </div>
                </div>
                // Command list
                <div class="space-y-3">
                    { commands.iter().map(|cmd| {
                            let aliases = cmd.aliases();

                            html! {
                                <div class="group hover:bg-base-200/50 p-3 rounded-lg transition-colors">
                                    <div class="flex items-baseline gap-3 mb-1">
                                        <span class="text-success font-mono font-semibold text-base">
                                            { cmd.name() }
                                        </span>

                                        {
                                            if !aliases.is_empty() {
                                                html! {
                                                    <span class="text-base-content/40 text-xs font-mono">
                                                        { "aliases: " }
                                                        { aliases.join(", ") }
                                                    </span>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>

                                    <div class="text-base-content/80 text-sm pl-4">
                                        { cmd.description() }
                                    </div>
                                </div>
                            }
                        }).collect::<Html>() }
                </div>
                // Footer with tips
                <div class="border-t border-primary/30 pt-4 mt-6">
                    <div class="text-base-content/60 text-sm space-y-2">
                        <div>
                            <span class="text-accent">{ "💡 Tip:" }</span>
                            { " Press " }
                            <kbd class="kbd kbd-sm">{ "↑" }</kbd>
                            { " and " }
                            <kbd class="kbd kbd-sm">{ "↓" }</kbd>
                            { " to navigate command history" }
                        </div>
                        <div>
                            <span class="text-accent">{ "💡 Tip:" }</span>
                            { " Press " }
                            <kbd class="kbd kbd-sm">{ "Tab" }</kbd>
                            { " for command auto-completion (coming soon)" }
                        </div>
                    </div>
                </div>
                // Back button
                <div class="flex justify-center pt-4">
                    <button
                        onclick={back_to_terminal}
                        class="btn btn-primary btn-sm gap-2 font-mono"
                    >
                        <span>{ "←" }</span>
                        <span>{ "Back to Terminal" }</span>
                    </button>
                </div>
            </div>
        }
    }
}

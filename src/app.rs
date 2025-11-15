use yew::prelude::*;

use yew::use_state;
use yew::Callback;

use crate::components::{FontSelector, ModalFrame, ThemeSwitcher};
use crate::terminal::{self, CommandOutput};
use crate::views;
use crate::windows;

#[function_component(App)]
pub fn app() -> Html {
    let active_window = use_state(|| None::<String>);

    let on_command_complete = {
        let active_window = active_window.clone();
        Callback::from(move |output: CommandOutput| {
            if let CommandOutput::Launch(window_id) = output {
                active_window.set(Some(window_id));
            }
        })
    };

    let on_close_window = {
        let active_window = active_window.clone();
        Callback::from(move |_| {
            active_window.set(None);
        })
    };

    html! {
        <div class="bg-base-100 text-base-content min-h-screen">
            <div class="fixed top-4 right-4 z-50 flex gap-4">
                <FontSelector />
                <ThemeSwitcher />
            </div>

            // The main terminal component
            <main class="p-4">

            <terminal::component::Terminal on_command_complete={on_command_complete} />

            // Static sections below the terminal
                <views::about::About />
                <views::skills::Skills />
                <views::experience::Experience />
                <views::projects::Projects />
                <views::contact::Contact />
            </main>

            // Conditionally rendered windows (modals)
            if let Some(window_id) = &*active_window {
                <ModalFrame on_close={on_close_window.clone()}>
                {
                    match window_id.as_str() {
                        "projects" => html!{ <windows::projects_window::ProjectsWindow /> },
                        "about" => html!{ <windows::about_window::AboutWindow /> },
                        "contact" => html!{ <windows::contact_window::ContactWindow /> },
                        _ => html!{}
                    }
                }
                </ModalFrame>
            }
        </div>
    }
}


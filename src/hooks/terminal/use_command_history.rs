use yew::prelude::*;

/// Hook for navigating command history with up/down arrows
#[hook]
pub fn use_command_history(history: Vec<String>) -> (Option<String>, Callback<String>) {
    let current_index = use_state(|| None::<usize>);

    let navigate = {
        let current_index = current_index.clone();
        Callback::from(move |direction: String| {
            // Implementation for up/down navigation
        })
    };

    let current_command = current_index
        .as_ref()
        .and_then(|&idx| history.get(idx))
        .cloned();

    (current_command, navigate)
}

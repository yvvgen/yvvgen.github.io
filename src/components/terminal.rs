use yew::prelude::*;

#[function_component(Terminal)]
pub fn terminal() -> Html {
    html! {
        <pre data-prefix="$">
            <code>{ "npm i daisyui" }</code>
        </pre>
    }
}

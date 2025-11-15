use yew::prelude::*;

#[function_component(AboutWindow)]
pub fn about_window() -> Html {
    html! {
        <div class="window about-window">
            <h2>{"About Me"}</h2>
            <p>{"This is where your rich about me UI will go."}</p>
            // TODO: Implement actual about me display
        </div>
    }
}

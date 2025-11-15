use yew::prelude::*;

#[function_component(ContactWindow)]
pub fn contact_window() -> Html {
    html! {
        <div class="window contact-window">
            <h2>{"Contact Me"}</h2>
            <p>{"This is where your rich contact UI will go."}</p>
            // TODO: Implement actual contact form/info
        </div>
    }
}

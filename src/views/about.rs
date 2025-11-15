use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="card-project">
            <div class="card-body">
                <h2 class="card-title">{"About Me"}</h2>
                <p>{"Passionate developer with experience in building modern web applications."}</p>
            </div>
        </div>
    }
}

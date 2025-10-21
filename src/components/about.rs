use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section class="max-w-4xl mx-auto px-4 py-16">
            <h2 class="text-4xl font-bold text-white mb-8 font-caskaydia">{"About Me"}</h2>
            <p class="text-gray-300 text-lg leading-relaxed">
                {"Passionate developer with experience in building modern web applications."}
            </p>
        </section>
    }
}

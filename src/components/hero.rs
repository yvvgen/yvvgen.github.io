use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="min-h-screen flex flex-col items-center justify-center text-center px-4 bg-gray-900">
            <h1 class="text-6xl md:text-8xl font-bold text-white mb-4 font-caskaydia animate-fade-in">
                {"Your Name"}
            </h1>
            <p class="text-2xl text-cyan-400 animate-slide-up">
                {"Full Stack Developer"}
            </p>
        </section>
    }
}

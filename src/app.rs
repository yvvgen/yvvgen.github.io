use yew::prelude::*;
use crate::components::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="bg-gray-900 text-white min-h-screen">
            <Hero />
            <About />
            <Skills />
            <Experience />
            <Projects />
            <Contact />
        </div>
    }
}

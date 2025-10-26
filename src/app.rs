use crate::components::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="bg-base-100 text-base-content min-h-screen">
            <div class="fixed top-4 right-4 z-50 flex gap-4">
                <FontSelector />
                <ThemeSwitcher />
            </div>
            <Hero />
            <main class="p-4">
                <About />
                <Skills />
                <Experience />
                <Projects />
                <Contact />
            </main>
        </div>
    }
}

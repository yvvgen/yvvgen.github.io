use yew::{function_component, html, Html};

// Import all components from the components module
use crate::components::{Contact, Experience, Hero, Navigation, Projects, Skills};
// Import data constant
use crate::data::profile::PROFILE;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="bg-base-100 bg-synthwave-grid">
            <Navigation />
            <main class="container mx-auto px-4">
                // The Hero component now receives a 'title' prop.
                <Hero title="Yvvgen" />
                <Experience />
                <Projects />
                <Skills />
                <Contact />
            </main>
            <footer class="bg-transparent text-white py-8 text-center">
                // Use the imported PROFILE data here
                <p>
                    { format!("© 2024 {}. Built with Rust & Yew. Interest: {}", PROFILE.name, PROFILE.interest) }
                </p>
            </footer>
        </div>
    }
}

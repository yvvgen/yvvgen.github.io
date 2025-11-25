use crate::components::{
    about::About, contact::Contact, experience::Experience, hero::Hero, navigation::Navigation,
    projects::Projects, skills::Skills,
};
use crate::components::loading::{
    about::About as LoadingAbout, contact::Contact as LoadingContact,
    experience::Experience as LoadingExperience, hero::Hero as LoadingHero,
    navigation::Navigation as LoadingNavigation, projects::Projects as LoadingProjects,
    skills::Skills as LoadingSkills,
};
use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
enum LoadingState {
    Loading,
    Booting,
    Loaded,
}

#[function_component(App)]
pub fn app() -> Html {
    let loading_state = use_state(|| LoadingState::Loading);

    {
        let loading_state_for_loading_timeout = loading_state.clone(); // Clone for the first timeout
        let loading_state_for_booting_timeout = loading_state.clone(); // Clone for the second timeout
        use_effect_with((), move |_| {
            let loading_timeout = Timeout::new(1000, move || {
                loading_state_for_loading_timeout.set(LoadingState::Booting);
            });

            let booting_timeout = Timeout::new(5000, move || {
                loading_state_for_booting_timeout.set(LoadingState::Loaded);
            });

            || {
                loading_timeout.cancel();
                booting_timeout.cancel();
            }
        });
    }

    let main_class = match *loading_state {
        LoadingState::Loaded => "flicker-in",
        _ => "",
    };

    html! {
        <div class="bg-base-100 bg-synthwave-grid">
            {
                match *loading_state {
                    LoadingState::Loading | LoadingState::Booting => html! {
                        <>
                            <LoadingNavigation />
                            <main class="container mx-auto px-4">
                                <LoadingHero booting={*loading_state == LoadingState::Booting} />
                                <LoadingExperience />
                                <LoadingProjects />
                                <LoadingSkills />
                                <div class="flex">
                                    <LoadingAbout />
                                    <LoadingContact />
                                </div>
                            </main>
                        </>
                    },
                    LoadingState::Loaded => html! {
                        <div class={main_class}>
                            <Navigation />
                            <main class="container mx-auto px-4">
                                <Hero title="Yvvgen" />
                                <Experience />
                                <Projects />
                                <Skills />
                                <div class="flex">
                                    <About />
                                    <Contact />
                                </div>
                            </main>
                            <footer class="bg-transparent text-white py-8 text-center">
                                <p>{ "© 2024 Yvvgen. Built with Rust & Yew." }</p>
                            </footer>
                        </div>
                    },
                }
            }
        </div>
    }
}

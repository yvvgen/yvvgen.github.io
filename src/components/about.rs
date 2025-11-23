use yew::{function_component, html, Html};
use crate::data::profile::PROFILE;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="py-20 card-synthwave text-base-content overflow-hidden mt-16">
            <div class="container mx-auto px-6 max-w-5xl">
                <h2 class="text-5xl font-extrabold text-center mb-12 text-neon-secondary">{"About Me"}</h2>
                <div class="flex flex-col lg:flex-row items-center lg:space-x-12">
                    // Profile/Avatar Placeholder
                    <div class="w-full lg:w-1/3 mb-8 lg:mb-0 transform hover:scale-105 transition duration-500">
                        <div class="avatar mx-auto block">
                            <div class="w-64 rounded-full shadow-2xl border-4 border-primary">
                                // Placeholder image: 
                                <img src="https://placehold.co/256x256/1A1A2E/E0BBE4?text=YV" alt="Profile Avatar" />
                            </div>
                        </div>
                        <p class="text-center mt-4 text-lg font-medium text-secondary">
                            { format!("Interest: {}", PROFILE.interest) }
                        </p>
                    </div>

                    // Text Content
                    <div class="w-full lg:w-2/3 space-y-6">
                        <p class="text-xl leading-relaxed">
                            { format!("Hello! I'm {}, a passionate developer focused on building robust and high-performance applications using Rust and WebAssembly.", PROFILE.name) }
                        </p>
                        <p class="text-xl leading-relaxed border-l-4 border-secondary pl-4 py-2 bg-base-100 p-4 rounded-lg shadow-md">
                            { format!("My current intellectual focus lies in the philosophy of {}. I explore how disciplined, intentional, and perhaps selective, engagement with modern technology can lead to better outcomes.", PROFILE.interest) }
                        </p>
                        <p class="text-lg leading-relaxed text-base-content/70">
                            {"This portfolio serves as a demonstration of my technical proficiency in the Yew framework, showcasing clean architecture, component design, and performance optimization."}
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

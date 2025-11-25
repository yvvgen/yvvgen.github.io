use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    html! {
        <section id="skills" class="py-20 text-base-content">
            <h2 class="text-6xl font-extrabold text-center mb-16 text-neon-secondary animate-pulse">
                { "Skills & Mastery" }
            </h2>
            <div class="container mx-auto px-6 max-w-4xl">
                <div class="grid grid-cols-2 md:grid-cols-3 gap-6">
                    { for (0..6).map(|_| html! {
                            <div class="p-4 card-synthwave rounded-lg shadow-xl animate-pulse">
                                <div class="h-8 w-3/4 bg-base-300/50 rounded mb-1"></div> // Skill Name
                                <div class="h-5 w-1/2 bg-base-300/50 rounded"></div> // Skill Level
                            </div>
                        }) }
                </div>
                <div class="mt-12 text-center">
                    <div class="btn btn-accent btn-outline text-xl h-14 w-60 bg-base-300/50 rounded animate-pulse"></div> // Download Resume Button
                </div>
            </div>
        </section>
    }
}

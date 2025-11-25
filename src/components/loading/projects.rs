use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id="projects" class="py-20 text-base-content">
            <h2 class="text-6xl font-extrabold text-center mb-16 text-neon-primary">
                { "Projects" }
            </h2>
            <div class="container mx-auto px-6">
                <div class="grid grid-cols-1 md::grid-cols-2 lg:grid-cols-3 gap-8">
                    { for (0..6).map(|_| html! {
                        <div class="card-synthwave shadow-xl animate-pulse">
                            <div class="card-body">
                                <div class="h-8 bg-base-300/50 rounded w-3/4 mb-4"></div>
                                <div class="h-4 bg-base-300/50 rounded w-full mb-2"></div>
                                <div class="h-4 bg-base-300/50 rounded w-5/6 mb-4"></div>
                                <div class="flex flex-wrap mt-4">
                                    <div class="h-6 bg-base-300/50 rounded-full w-16 mr-2 mb-2"></div>
                                    <div class="h-6 bg-base-300/50 rounded-full w-20 mr-2 mb-2"></div>
                                    <div class="h-6 bg-base-300/50 rounded-full w-12 mr-2 mb-2"></div>
                                </div>
                            </div>
                        </div>
                    }) }
                </div>
            </div>
        </section>
    }
}

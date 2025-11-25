use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="py-20 text-base-content">
            <h2 class="text-5xl font-extrabold text-center mb-12 text-neon-primary animate-pulse">
                { "Get In Touch" }
            </h2>
            <div class="container mx-auto px-6 max-w-xl">
                <div class="card-synthwave shadow-2xl p-8 animate-pulse">
                    <div class="h-6 w-3/4 bg-base-300/50 rounded mb-6 mx-auto"></div> // Intro paragraph
                    <div class="space-y-4">
                        <div class="form-control w-full">
                            <div class="label"><div class="label-text h-4 w-20 bg-base-300/50 rounded"></div></div>
                            <div class="input input-bordered w-full h-12 bg-base-300/50 rounded"></div>
                        </div>
                        <div class="form-control w-full">
                            <div class="label"><div class="label-text h-4 w-20 bg-base-300/50 rounded"></div></div>
                            <div class="input input-bordered w-full h-12 bg-base-300/50 rounded"></div>
                        </div>
                        <div class="form-control w-full">
                            <div class="label"><div class="label-text h-4 w-20 bg-base-300/50 rounded"></div></div>
                            <div class="textarea textarea-bordered h-24 bg-base-300/50 rounded"></div>
                        </div>
                        <div class="btn btn-primary w-full h-12 bg-base-300/50 rounded"></div> // Submit button
                    </div>
                </div>
            </div>
        </section>
    }
}

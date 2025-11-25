use crate::data::profile_data::Profile;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let profile = Profile::get();

    html! {
        <section id="about" class="py-20 min-h-screen flex items-center justify-center ">
            <div class="container mx-auto px-4">
                <div class="card card-synthwave lg:card-side scanlines">
                    <div class="card-body md:w-full">
                        <h2 class="card-title text-4xl font-display text-neon-primary mb-4">
                            { "About Me" }
                        </h2>
                        <div class="space-y-2 text-left mb-6">
                            <h3 class="text-2xl font-bold font-display">
                                { &profile.identity.name }
                            </h3>
                            <p class="text-lg text-primary">{ &profile.identity.title }</p>
                            <p class="text-base-content opacity-70">
                                { &profile.identity.location }
                            </p>
                        </div>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
                            <div>
                                <h3 class="text-xl font-bold font-mono text-neon-secondary mb-3">
                                    { "// Contact" }
                                </h3>
                                <p class="mb-2 font-mono text-sm">
                                    <b>{ "Phone: " }</b>
                                    { &profile.contact.phone }
                                </p>
                                <p class="mb-4 font-mono text-sm">
                                    <b>{ "Email: " }</b>
                                    { &profile.contact.email }
                                </p>
                            </div>
                            <div>
                                <h3 class="text-xl font-bold font-mono text-neon-secondary mb-3">
                                    { "// Education" }
                                </h3>
                                <ul class="space-y-2">
                                    { for profile.education.iter().map(|edu| html!{
                                        <li class="font-mono text-sm">
                                            <b>{&edu.institution}</b>{": "}{&edu.degree}{" ("}{&edu.date_range}{")"}
                                        </li>
                                    }) }
                                </ul>
                            </div>
                            <div class="md:col-span-2">
                                <h3 class="text-xl font-bold font-mono text-neon-secondary mb-3">
                                    { "// Languages" }
                                </h3>
                                <ul class="flex flex-wrap gap-2">
                                    { for profile.languages.iter().map(|lang| html!{
                                        <li class="badge badge-soft-primary badge-synthwave">{&lang.language}{" - "}{&lang.level}</li>
                                    }) }
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

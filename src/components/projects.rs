use crate::data::ProjectItem;
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = use_state(ProjectItem::get_all);
    let selected_project = use_state(|| None);

    let on_project_select = {
        let selected_project = selected_project.clone();
        Callback::from(move |project: ProjectItem| {
            selected_project.set(Some(project));
        })
    };

    let on_back_to_grid = {
        let selected_project = selected_project.clone();
        Callback::from(move |_| {
            selected_project.set(None);
        })
    };

    html! {
        <section id="projects" class="py-20 text-base-content">
            <h2 class="text-6xl font-extrabold text-center mb-16 text-neon-primary">
                { "Projects" }
            </h2>
            <div class="container mx-auto px-6">
                { if let Some(project) = &*selected_project {
                        // Detailed view
                        html! {
                            <div class="card-synthwave shadow-xl p-8">
                                <div class="flex justify-between items-start">
                                    <div>
                                        <h3 class="font-bold text-4xl text-neon-accent mb-4">{&project.title}</h3>
                                        <p class="py-2 text-lg font-mono italic">{&project.date_range}</p>
                                    </div>
                                    <button onclick={on_back_to_grid.clone()} class="btn btn-accent btn-outline transition duration-300 transform hover:scale-105 hover:glow-accent">{"< Back"}</button>
                                </div>

                                { for project.description.iter().map(|d| html! { <p class="py-2 text-base-content/90 text-xl">{d}</p> }) }

                                <div class="py-4 mt-6">
                                    <h4 class="text-2xl font-semibold text-neon-secondary mb-3">{"Technologies Used"}</h4>
                                    <div class="card-actions justify-start flex-wrap">
                                        { for project.skills.iter().map(|skill| html!{
                                            <div class="badge badge-soft-secondary badge-synthwave text-lg mr-2 mb-2">{&skill.name}</div>
                                        })}
                                    </div>
                                </div>
                                <div class="card-actions mt-6">
                                    <a href="#" class="btn btn-primary transition duration-300 transform hover:scale-105 hover:glow-primary">{"View Source"}</a>
                                </div>
                            </div>
                        }
                    } else {
                        // Grid view
                        html! {
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                                {
                                    projects.iter().map(|project| {
                                        let on_project_select = on_project_select.clone();
                                        let project_clone = project.clone();
                                        html! {
                                            <div onclick={move |_| on_project_select.emit(project_clone.clone())}
                                                 class="card-synthwave shadow-xl hover:shadow-2xl transition duration-500 transform hover:-translate-y-2 cursor-pointer">
                                                <div class="card-body">
                                                    <h3 class="card-title text-2xl font-bold text-neon-accent">{&project.title}</h3>
                                                    <p class="text-base-content/80">{project.description.first().cloned().unwrap_or_default()}</p>
                                                    <div class="card-actions justify-start mt-4 flex-wrap">
                                                        { for project.skills.iter().take(3).map(|skill| html!{
                                                            <div class="badge badge-soft-primary badge-synthwave mr-2 mb-2">{&skill.name}</div>
                                                        })}
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    } }
            </div>
        </section>
    }
}

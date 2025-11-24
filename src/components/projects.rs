use yew::{function_component, html, Html};

// Define a Project structure
struct Project {
    title: &'static str,
    description: &'static str,
    tags: Vec<&'static str>,
}

fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Yew Dashboard",
            description: "A comprehensive analytics dashboard built entirely with Yew and Tailwind.",
            tags: vec!["Yew", "Tailwind", "Wasm"],
        },
        Project {
            title: "Rust CLI Tool",
            description: "A performant command-line interface tool for file processing, emphasizing speed.",
            tags: vec!["Rust", "CLI", "Performance"],
        },
        Project {
            title: "Decentralized Blog",
            description: "An experimental content platform using Rust's capabilities for cryptographic signing.",
            tags: vec!["Security", "Blockchain", "Rust"],
        },
    ]
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = get_projects();

    html! {
        <section id="projects" class="py-20 bg-base-100 text-base-content">
            <h2 class="text-6xl font-extrabold text-center mb-16 text-neon-primary">{"Featured Projects"}</h2>
            <div class="container mx-auto px-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10">
                {
                    projects.iter().map(|project| html! {
                        <div class="card-synthwave shadow-xl hover:shadow-2xl transition duration-500 transform hover:-translate-y-2">
                            <div class="card-body">
                                <h3 class="card-title text-3xl font-bold text-neon-accent">{ &project.title }</h3>
                                <p class="text-xl text-base-content/80">{ &project.description }</p>
                                <div class="card-actions justify-end mt-4">
                                    {
                                        project.tags.iter().map(|tag| html! {
                                            <div class="badge badge-secondary text-lg cursor-default">
                                                { tag }
                                            </div>
                                        }).collect::<Html>()
                                    }
                                </div>
                                <a href="#" class="mt-4 link link-hover text-base text-accent font-medium hover:text-neon-accent">{"View Source »"}</a>
                            </div>
                        </div>
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}

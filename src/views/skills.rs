use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let skills = vec![
        "Rust",
        "TypeScript",
        "React",
        "Yew",
        "WebAssembly",
        "Tailwind",
    ];

    html! {
        <div class="card-project">
            <div class="card-body">
                <h2 class="card-title">{"Skills"}</h2>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 pt-4">
                    {
                        skills.iter().map(|skill| {
                            html! {
                                <div class="card bg-base-100 shadow-md hover:shadow-lg transition-shadow duration-300">
                                    <div class="card-body items-center text-center">
                                        <h3 class="card-title">{skill}</h3>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}

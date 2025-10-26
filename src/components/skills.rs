use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let skills = vec!["Rust", "TypeScript", "React", "Yew", "WebAssembly", "Tailwind"];
    
    html! {
        <div class="card bg-base-200 shadow-xl">
            <div class="card-body">
                <h2 class="card-title">{"Skills"}</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {
                        skills.iter().map(|skill| {
                            html! {
                                <div class="card bg-base-100 shadow-md">
                                    <div class="card-body">
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

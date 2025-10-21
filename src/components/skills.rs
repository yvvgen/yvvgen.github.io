use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let skills = vec!["Rust", "TypeScript", "React", "Yew", "WebAssembly", "Tailwind"];
    
    html! {
        <section class="max-w-6xl mx-auto px-4 py-16">
            <h2 class="text-4xl font-bold text-white mb-12 text-center font-caskaydia">{"Skills"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {
                    skills.iter().map(|skill| {
                        html! {
                            <div class="bg-gray-800 rounded-lg p-6 hover:scale-105 hover:shadow-cyan-500/50 shadow-lg transition-all duration-300">
                                <h3 class="text-xl font-bold text-cyan-400">{skill}</h3>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </section>
    }
}

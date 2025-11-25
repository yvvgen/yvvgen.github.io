use yew::{function_component, html, Html};

fn get_skills() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Rust", "Expert"),
        ("Yew Framework", "Advanced"),
        ("Tailwind CSS / DaisyUI", "Advanced"),
        ("WebAssembly (Wasm)", "Intermediate"),
        ("Git & CI/CD", "Advanced"),
        ("PostgreSQL / SQL", "Intermediate"),
        ("Architecture Design", "Advanced"),
    ]
}

#[function_component(Skills)]
pub fn skills() -> Html {
    let skills = get_skills();
    html! {
        <section id="skills" class="py-20 text-base-content">
            <h2 class="text-6xl font-extrabold text-center mb-16 text-neon-secondary">
                { "Skills & Mastery" }
            </h2>
            <div class="container mx-auto px-6 max-w-4xl">
                <div class="grid grid-cols-2 md:grid-cols-3 gap-6">
                    { skills.iter().map(|(name, level)| html! {
                            <div class="p-4 card-synthwave rounded-lg shadow-xl hover:shadow-primary/50 transition duration-300 transform hover:scale-[1.02]">
                                <p class="text-2xl font-semibold mb-1">{ name }</p>
                                <div class="badge badge-md text-lg text-base-content/70">{ level }</div>
                            </div>
                        }).collect::<Html>() }
                </div>
                <div class="mt-12 text-center">
                    <button
                        class="btn btn-accent btn-outline text-xl transition duration-300 transform hover:scale-105 hover:glow-accent"
                    >
                        { "Download Resume" }
                    </button>
                </div>
            </div>
        </section>
    }
}

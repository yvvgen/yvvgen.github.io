use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub subtitle: AttrValue,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <div
            id="hero"
            class="hero scanlines font-terminal min-h-[70vh] bg-gradient-synthwave-reverse text-primary-content shadow-2xl rounded-lg border-4 border-accent"
        >
            <div class="hero-content text-center py-20">
                <div class="max-w-3xl">
                    <h1 class="text-6xl font-terminal font-extrabold mb-4 text-neon-accent">
                        { &props.title }
                    </h1>
                    <p class="text-xl opacity-90 mb-8">
                        { if props.subtitle.is_empty() {
                                "A Rust and Yew Developer."
                            } else {
                                &props.subtitle
                            } }
                    </p>
                    <a
                        href="#projects"
                        class="btn btn-lg btn-accent shadow-lg hover:shadow-xl transition duration-300 transform hover:scale-105 hover:glow-accent"
                    >
                        { "Explore My Work" }
                    </a>
                </div>
            </div>
        </div>
    }
}

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
            class="scanlines mt-8 font-terminal min-h-[70vh] bg-gradient-synthwave-reverse text-primary-content shadow-2xl rounded-lg border-4 border-accent accent-glow"
        >
            <div class="flex items-center text-accent mb-4 mt-4 ml-8 mr-8">
                <span class="mr-2 text-warning">{ "$" }</span>
                <h1 class="font-extrabold text-neon-accent leading-none">{ &props.title }</h1>
            </div>
            <div class="flex items-center text-3xl opacity-90 mb-8">
                <span class="mr-2 text-success" />
                <p>
                    { if props.subtitle.is_empty() {
                                    "Break all the machines"
                                } else {
                                    &props.subtitle
                                } }
                </p>
            </div>
            <div class="flex items-center">
                <span class="mr-2 text-info" />
                <a
                    href="#projects"
                    class="btn btn-lg btn-accent shadow-lg hover:shadow-xl transition duration-300 transform hover:scale-105 hover:glow-accent"
                >
                    { "Explore My Work" }
                </a>
            </div>
        </div>
    }
}

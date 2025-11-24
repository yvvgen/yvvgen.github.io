use crate::data::experience::{ExperienceItem, TimelinePosition};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExperienceProps {
    #[prop_or_default]
    pub experiences: Option<Vec<ExperienceItem>>,
}

#[function_component(Experience)]
pub fn experience(props: &ExperienceProps) -> Html {
    let experiences = props
        .experiences
        .clone()
        .unwrap_or_else(|| ExperienceItem::get_all());

    html! {
        <section id="experience" class="min-h-screen py-16 px-4 sm:px-6 lg:px-8">
            <div class="max-w-7xl mx-auto">
                <h1 class="text-5xl font-bold text-center mb-16 text-neon-primary">
                    { "Experience" }
                </h1>
                <div class="relative">
                    // Central timeline line
                    <div
                        class="absolute left-1/2 transform -translate-x-1/2 h-full w-1 bg-gradient-to-b from-primary via-accent to-secondary"
                    />
                    // Experience items
                    <div class="space-y-24">
                        { experiences.iter().enumerate().map(|(index, exp)| {
                            render_experience_item(exp, index)
                        }).collect::<Html>() }
                    </div>
                </div>
            </div>
        </section>
    }
}

fn render_experience_item(experience: &ExperienceItem, index: usize) -> Html {
    let (container_class, card_class, timeline_dot_class) = match experience.position {
        TimelinePosition::Left => (
            "flex justify-start pr-8 md:pr-16",
            "w-full md:w-5/12",
            "absolute left-1/2 transform -translate-x-1/2 -translate-y-1/2 top-0",
        ),
        TimelinePosition::Right => (
            "flex justify-end pl-8 md:pl-16",
            "w-full md:w-5/12 ml-auto",
            "absolute left-1/2 transform -translate-x-1/2 -translate-y-1/2 top-0",
        ),
    };

    // Alternate colors for dots
    let dot_color = match index % 3 {
        0 => "bg-secondary glow-secondary",
        1 => "bg-primary glow-primary",
        _ => "bg-accent glow-accent",
    };

    // Extract year from date range
    let year = experience
        .date_range
        .split_whitespace()
        .find(|word| word.len() == 4 && word.chars().all(|c| c.is_numeric()))
        .unwrap_or("2024");

    // Unique ID for collapse control
    let collapse_id = format!("collapse-{}", index);

    html! {
        <div class="relative">
            // Timeline dot with year
            <div class={classes!(timeline_dot_class)}>
                <label
                    for={collapse_id.clone()}
                    class={classes!("flex", "items-center", "justify-center", "w-16", "h-16", "rounded-full", dot_color, "border-4", "border-base-100", "z-10", "cursor-pointer", "transition-all", "duration-300", "hover:scale-110")}
                >
                    <span class="text-xs font-bold font-mono">{ year }</span>
                </label>
            </div>
            // Experience card container
            <div class={classes!(container_class)}>
                <div class={classes!(card_class)}>
                    <div
                        class="collapse collapse-arrow card-synthwave transition-all duration-300 hover:glow-primary"
                    >
                        <input type="checkbox" id={collapse_id} class="peer" />
                        <div class="collapse-title p-6 pb-2">
                            // Header (always visible)
                            <div class="flex justify-between items-start flex-wrap gap-2">
                                <div class="flex-1">
                                    <h2 class="text-xl font-bold text-primary mb-1">
                                        { &experience.title }
                                    </h2>
                                    <p class="text-secondary font-semibold">
                                        { &experience.company }
                                    </p>
                                </div>
                                <span class="text-sm text-accent font-mono whitespace-nowrap">
                                    { &experience.date_range }
                                </span>
                            </div>
                        </div>
                        <div class="collapse-content px-6">
                            // Responsibilities (expandable)
                            <ul class="space-y-2 text-base-content mt-2">
                                { experience.responsibilities.iter().map(|resp| html! {
                                    <li class="flex items-start">
                                        <span class="text-primary mr-2 mt-1">{"▸"}</span>
                                        <span class="leading-relaxed">{resp}</span>
                                    </li>
                                }).collect::<Html>() }
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

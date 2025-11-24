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

    let total_count = experiences.len();

    html! {
        <div class="container mx-auto px-4 py-10">
            <h2 class="text-4xl font-display text-center mb-12 text-neon-primary animate-pulse">
                { "Experience Log" }
            </h2>
            <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
                { experiences.iter().enumerate().map(|(index, exp)| {
                    render_experience_item(exp, index, total_count)
                }).collect::<Html>() }
            </ul>
        </div>
    }
}

fn render_experience_item(experience: &ExperienceItem, index: usize, total_count: usize) -> Html {
    // 1. Determine Position
    let (content_class, date_class, content_align) = match experience.position {
        TimelinePosition::Left => (
            "timeline-start md:text-end",
            "timeline-end",
            "md:items-end", // Align flex items to end for badges
        ),
        TimelinePosition::Right => ("timeline-end", "timeline-start", "items-start"),
    };

    // 2. Cycle Neon Colors (Secondary -> Primary -> Accent)
    // We get specific badge classes for the cycle
    let (text_color, bg_color, border_color, badge_style, glyph_char) = match index % 3 {
        0 => (
            "text-secondary",
            "bg-secondary",
            "border-secondary",
            "badge-soft-secondary",
            "ɔ",
        ),
        1 => (
            "text-primary",
            "bg-primary",
            "border-primary",
            "badge-soft-primary",
            "ɔ",
        ),
        _ => (
            "text-accent",
            "bg-accent",
            "border-accent",
            "badge-soft-accent",
            "ɔ",
        ),
    };

    html! {
        <li>
            // --- TOP CONNECTOR ---
            if index > 0 {
                <hr class={bg_color} />
            }
            // --- DATE DISPLAY ---
            <div
                class={classes!(date_class, "mb-10")}
            >
                <div class={classes!("timeline-box", "ghost", "scanlines", text_color)}>
                    { &experience.date_range }
                </div>
            </div>
            // --- CENTER ICON (ANAKRON GLYPH) ---
            <div class="timeline-middle">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    class={classes!("w-5", "h-5", text_color)}
                >
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                        clip-rule="evenodd"
                    />
                </svg>
            </div>
            // --- MAIN CONTENT CARD ---
            <div
                class={classes!(content_class, "mb-10", "flex", "flex-col", "gap-2", "scanlines", content_align)}
            >
                <div
                    class={classes!("timeline-box", border_color, "p-6", "bg-opacity-20", "backdrop-blur-sm")}
                >
                    // HEADER
                    <div class="font-bold text-lg font-display tracking-wide">
                        { &experience.title }
                    </div>
                    <div class={classes!("text-sm", "opacity-80", "font-mono", text_color)}>
                        { &experience.company }
                    </div>
                    // --- NEW: SOFT BADGES ROW ---
                    <div
                        class={classes!("flex", "flex-wrap", "gap-2", "mt-3", "mb-2", if experience.position == TimelinePosition::Left { "md:justify-end" } else { "justify-start" })}
                    >
                        { experience.tags.iter().map(|tag| html! {
                            <span class={classes!("badge", "badge-synthwave", badge_style)}>
                                { tag }
                            </span>
                        }).collect::<Html>() }
                    </div>
                    // BODY
                    <div
                        class="collapse collapse-arrow border border-base-300 bg-base-200/30 rounded-box mt-2"
                    >
                        <input type="checkbox" />
                        <div
                            class={classes!("collapse-title", "text-sm", "font-medium", text_color)}
                        >
                            { "Details_Log" }
                        </div>
                        <div class="collapse-content text-left">
                            <ul class="list-none space-y-2 mt-2 text-xs md:text-sm">
                                { experience.responsibilities.iter().map(|resp| html! {
                                    <li class="flex items-start gap-2">
                                        <span class={classes!("mt-1", text_color)}>{"▸"}</span>
                                        <span>{resp}</span>
                                    </li>
                                }).collect::<Html>() }
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
            // --- BOTTOM CONNECTOR ---
            if index < total_count - 1 {
                <hr class={bg_color} />
            }
        </li>
    }
}

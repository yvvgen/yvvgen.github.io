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
        <section class="w-full py-10 px-4">
            <h2 class="text-3xl font-bold text-center mb-10 text-neon-primary">
                { "Experience Log" }
            </h2>
            <ul class="timeline timeline-vertical timeline-snap-icon max-md:timeline-compact">
                { experiences.iter().enumerate().map(|(index, exp)| {
                    render_experience_item(exp, index, total_count)
                }).collect::<Html>() }
            </ul>
        </section>
    }
}

fn render_experience_item(experience: &ExperienceItem, index: usize, total_count: usize) -> Html {
    // 1. Determine Position based on the Enum
    // timeline-start = Left side (on desktop)
    // timeline-end   = Right side (on desktop)
    let (content_class, date_class, content_align) = match experience.position {
        TimelinePosition::Left => (
            "timeline-start", // Content on Left
            "timeline-end",   // Date on Right
            "md:text-end",    // Align text towards the center line
        ),
        TimelinePosition::Right => (
            "timeline-end",   // Content on Right
            "timeline-start", // Date on Left
            "text-start",     // Align text standard
        ),
    };

    // 2. Cycle Neon Colors (Secondary -> Primary -> Accent)
    let (text_color, bg_color, border_color) = match index % 3 {
        0 => ("text-secondary", "bg-secondary", "border-secondary"),
        1 => ("text-primary", "bg-primary", "border-primary"),
        _ => ("text-accent", "bg-accent", "border-accent"),
    };

    html! {
        <li>
            // --- TOP CONNECTOR (The Neon Tube) ---
            if index > 0 {
                <hr class={bg_color} />
            }
            // --- DATE DISPLAY ("Ghost" style) ---
            // We place this opposite to the content
            <div
                class={classes!("timeline-box", "ghost", date_class)}
            >
                <span class={classes!("font-mono", "font-bold", "text-lg", "opacity-70")}>
                    { &experience.date_range }
                </span>
            </div>
            // --- CENTER ICON ---
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
                class={classes!(
                "timeline-box",
                "p-0", "w-full", "md:w-96", "overflow-hidden",
                content_class,
                content_align
            )}
            >
                // We use collapse-arrow to allow expanding details
                <div class="collapse collapse-arrow bg-transparent">
                    <input type="checkbox" />
                    // HEADER: Title & Company
                    <div class="collapse-title font-medium p-4">
                        <div
                            class={classes!("text-xl", "font-display", "tracking-wide", text_color)}
                        >
                            { &experience.title }
                        </div>
                        <div class="text-sm opacity-80 font-mono mt-1">{ &experience.company }</div>
                    </div>
                    // BODY: Responsibilities (Holo-data look)
                    <div class="collapse-content text-sm">
                        <ul class="list-none space-y-2 pb-2">
                            { experience.responsibilities.iter().map(|resp| html! {
                                <li class="flex items-start gap-2">
                                    // Bullet point matching the cycle color
                                    <span class={classes!("mt-[2px]", text_color)}>{"▸"}</span>
                                    <span class="opacity-90">{resp}</span>
                                </li>
                            }).collect::<Html>() }
                        </ul>
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

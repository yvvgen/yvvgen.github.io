use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <div id="experience" class="container mx-auto px-4 py-10">
            <h2 class="text-4xl font-display text-center mb-12 text-neon-primary animate-pulse">
                { "Experience " }
            </h2>
            <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
                { for (0..3).map(|i| render_placeholder_item(i)) }
            </ul>
        </div>
    }
}

fn render_placeholder_item(index: usize) -> Html {
    let (content_class, date_class) = if index % 2 == 0 {
        ("timeline-start md:text-end", "timeline-end")
    } else {
        ("timeline-end", "timeline-start")
    };

    let (text_color, bg_color) = match index % 3 {
        0 => ("text-secondary", "bg-secondary"),
        1 => ("text-primary", "bg-primary"),
        _ => ("text-accent", "bg-accent"),
    };

    html! {
        <li>
            if index > 0 {
                <hr class={classes!(bg_color, "opacity-30")} />
            }
            <div class={classes!(date_class, "mb-10")}>
                <div class="h-6 w-32 bg-base-300/50 rounded animate-pulse mx-auto md:mx-0"></div>
            </div>
            <div class="timeline-middle">
                <div class={classes!("w-5", "h-5", "rounded-full", bg_color, "opacity-30")}></div>
            </div>
            <div class={classes!(content_class, "mb-10")}>
                <div class="p-6 rounded-lg bg-base-200/30 w-full max-w-md animate-pulse">
                    <div class="h-6 w-3/4 bg-base-300/50 rounded mb-2"></div>
                    <div class="h-4 w-1/2 bg-base-300/50 rounded mb-4"></div>
                    <div class="flex flex-wrap gap-2 mb-4">
                        <div class="h-5 w-16 bg-base-300/50 rounded-full"></div>
                        <div class="h-5 w-20 bg-base-300/50 rounded-full"></div>
                        <div class="h-5 w-12 bg-base-300/50 rounded-full"></div>
                    </div>
                    <div class="h-10 w-full bg-base-300/50 rounded"></div>
                </div>
            </div>
            if index < 2 {
                <hr class={classes!(bg_color, "opacity-30")} />
            }
        </li>
    }
}

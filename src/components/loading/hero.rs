use gloo_timers::callback::Interval;
use yew::prelude::*;

const BOOT_MESSAGES: &[&str] = &[
    "Arch Linux (Yvvgen) -- (tty1)",
    "Starting systemd-udevd version 255.4-1-arch...",
    "Loading kernel modules...",
    "Initializing random seed...",
    "Mounting /dev/nvme0n1p2 on /home...",
    "Starting NetworkManager...",
    "Starting display manager: lightdm...",
    "Activating swap...",
    "Reaching for the stars... done.",
    "System startup finished in 3.421s.",
];

#[derive(Properties, PartialEq)]
pub struct Props {
    pub booting: bool,
}

#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    let lines = use_state(Vec::new);
    let is_booting_animation_active = use_state(|| false);

    // This effect runs only when props.booting becomes true and the animation hasn't started yet.
    if props.booting && !*is_booting_animation_active {
        is_booting_animation_active.set(true);
        let lines = lines.clone();
        let mut boot_message_index = 0;
        let interval = Interval::new(150, move || {
            if boot_message_index < BOOT_MESSAGES.len() {
                let mut current_lines_vec = (*lines).clone();
                current_lines_vec.push(BOOT_MESSAGES[boot_message_index]);
                lines.set(current_lines_vec);
                boot_message_index += 1;
            }
        });
        interval.forget();
    }

    html! {
        if props.booting {
            // Arch Linux Boot Animation
            <section class="min-h-[70vh] flex flex-col justify-center items-center text-white">
                <div class="w-full max-w-4xl p-4 bg-black bg-opacity-75 rounded-lg font-mono">
                    <div class="w-full h-64 overflow-y-auto text-green-400">
                        { for (*lines).iter().map(|line| html! { <p>{*line}</p> }) }
                    </div>
                </div>
            </section>
        } else {
            // Skeleton mimicking the actual Hero component when not booting
            <div
                id="hero"
                class="scanlines mt-8 font-terminal min-h-[70vh] bg-gradient-synthwave-reverse text-primary-content shadow-2xl rounded-lg border-4 border-accent accent-glow animate-pulse"
            >
                <div class="flex items-center text-accent mb-4 mt-4 ml-8 mr-8">
                    <span class="mr-2 text-warning">{ "$" }</span>
                    <div class="h-10 w-64 bg-base-300/50 rounded"></div> // Title placeholder
                </div>
                <div class="flex items-center text-3xl opacity-90 mb-8">
                    <span class="mr-2 text-success" />
                    <div class="h-6 w-96 bg-base-300/50 rounded"></div> // Subtitle placeholder
                </div>
                <div class="flex items-center">
                    <span class="mr-2 text-info" />
                    <div class="h-12 w-48 bg-base-300/50 rounded"></div> // Button placeholder
                </div>
            </div>
        }
    }
}

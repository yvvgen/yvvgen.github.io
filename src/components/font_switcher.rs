use yew::prelude::*;
use web_sys::window;

#[derive(Clone, PartialEq)]
pub struct FontOption {
    pub name: &'static str,
    pub family: &'static str,
    pub google_fonts_url: &'static str,
    pub description: &'static str,
}

const FONTS: &[FontOption] = &[
    FontOption {
        name: "Inter",
        family: "'Inter', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap",
        description: "Modern, versatile sans-serif",
    },
    FontOption {
        name: "IBM Plex Sans",
        family: "'IBM Plex Sans', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600;700&display=swap",
        description: "Professional, technical elegance",
    },
    FontOption {
        name: "Space Grotesk",
        family: "'Space Grotesk', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&display=swap",
        description: "Geometric, contemporary style",
    },
    FontOption {
        name: "Manrope",
        family: "'Manrope', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700;800&display=swap",
        description: "Clean, modern geometric sans",
    },
    FontOption {
        name: "DM Sans",
        family: "'DM Sans', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;700&display=swap",
        description: "Low-contrast, highly legible",
    },
    FontOption {
        name: "Plus Jakarta Sans",
        family: "'Plus Jakarta Sans', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap",
        description: "Friendly, rounded geometric",
    },
    FontOption {
        name: "Lexend",
        family: "'Lexend', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Lexend:wght@400;500;600;700&display=swap",
        description: "Optimized for reading efficiency",
    },
    FontOption {
        name: "Sora",
        family: "'Sora', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Sora:wght@400;500;600;700;800&display=swap",
        description: "Sharp, tech-inspired design",
    },
    FontOption {
        name: "Outfit",
        family: "'Outfit', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700;800&display=swap",
        description: "Rounded, approachable sans",
    },
    FontOption {
        name: "Work Sans",
        family: "'Work Sans', sans-serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Work+Sans:wght@400;500;600;700&display=swap",
        description: "Optimized for screen reading",
    },
    FontOption {
        name: "Crimson Pro",
        family: "'Crimson Pro', serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Crimson+Pro:wght@400;600;700&display=swap",
        description: "Classic serif with warmth",
    },
    FontOption {
        name: "Lora",
        family: "'Lora', serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Lora:wght@400;600;700&display=swap",
        description: "Elegant, calligraphic serif",
    },
    FontOption {
        name: "Spectral",
        family: "'Spectral', serif",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Spectral:wght@400;600;700&display=swap",
        description: "Refined serif for long reading",
    },
    FontOption {
        name: "JetBrains Mono",
        family: "'JetBrains Mono', monospace",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&display=swap",
        description: "Developer-friendly monospace",
    },
    FontOption {
        name: "Fira Code",
        family: "'Fira Code', monospace",
        google_fonts_url: "https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;500;600;700&display=swap",
        description: "Monospace with ligatures",
    },
    FontOption {
        name: "Caskaydia Cove Nerd Font",
        family: "'CaskaydiaCove Nerd Font', 'Cascadia Code', monospace",
        google_fonts_url: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css",
        description: "Cascadia Code with nerd icons",
    },
];

#[function_component(FontSelector)]
pub fn font_selector() -> Html {
    let selected_font = use_state(|| 0_usize);
    
    // Load the initial font
    use_effect_with((), move |_| {
        load_font(&FONTS[15]);
        apply_font(&FONTS[15]);
        || ()
    });

    let on_font_select = {
        let selected_font = selected_font.clone();
        Callback::from(move |index: usize| {
            selected_font.set(index);
            load_font(&FONTS[index]);
            apply_font(&FONTS[index]);
        })
    };

    html! {
        <div class="dropdown dropdown-bottom">
            <label tabindex="0" class="btn btn-ghost">{"Font"}</label>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow-lg bg-base-200 rounded-box w-80 mt-4 max-h-96 overflow-y-auto">
                <li class="menu-title">
                    <span>{"Choose Font"}</span>
                </li>
                {FONTS.iter().enumerate().map(|(i, font)| {
                    let is_active = i == *selected_font;
                    let on_click = {
                        let on_font_select = on_font_select.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_font_select.emit(i);
                        })
                    };
                    
                    html! {
                        <li key={i}>
                            <a 
                                onclick={on_click}
                                class={classes!(is_active.then(|| "active"))}
                            >
                                <div class="flex flex-col items-start w-full">
                                    <span class="font-semibold">{font.name}</span>
                                    <span class="text-xs opacity-60">{font.description}</span>
                                </div>
                            </a>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

fn load_font(font: &FontOption) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(head) = document.head() {
                // Remove old font links
                let existing_links = document.query_selector_all("link[data-font-loader]");
                if let Ok(links) = existing_links {
                    for i in 0..links.length() {
                        if let Some(link) = links.get(i) {
                            let _ = head.remove_child(&link);
                        }
                    }
                }
                
                // Add new font link
                if let Ok(link) = document.create_element("link") {
                    link.set_attribute("rel", "stylesheet").ok();
                    link.set_attribute("href", font.google_fonts_url).ok();
                    link.set_attribute("data-font-loader", "true").ok();
                    let _ = head.append_child(&link);
                }
            }
        }
    }
}

fn apply_font(font: &FontOption) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let style_value = format!("font-family: {}", font.family);
                let _ = body.set_attribute("style", &style_value);
            }
        }
    }
}

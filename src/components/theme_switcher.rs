use web_sys::{window, HtmlSelectElement};
use yew::prelude::*;

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let current_theme = use_state(|| "terminal".to_string());

    // Initialize theme on mount
    use_effect_with((), {
        let current_theme = current_theme.clone();
        move |_| {
            // Get theme from localStorage or use default
            if let Some(window) = window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved_theme)) = storage.get_item("theme") {
                        set_html_theme(&saved_theme);
                        current_theme.set(saved_theme);
                    } else {
                        set_html_theme("synthwave-violet");
                    }
                }
            }
            || ()
        }
    });

    let _on_change = {
        let current_theme = current_theme.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = target {
                let new_theme = select.value();
                set_html_theme(&new_theme);
                current_theme.set(new_theme.clone());

                // Save to localStorage
                if let Some(window) = window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("theme", &new_theme);
                    }
                }
            }
        })
    };

    html! {
        <div class="dropdown dropdown-end">
            <label tabindex="0" class="btn btn-ghost btn-circle">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-5 h-5 stroke-current">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
                </svg>
            </label>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow-lg bg-base-200 rounded-box w-160 mt-4 max-h-96 overflow-y-auto">
                <li class="menu-title">
                    <span>{"Choose Theme"}</span>
                </li>
                {themes().iter().map(|(name, label)| {
                    let is_active = *current_theme == *name;
                    let theme_name = name.to_string();
                    let on_click = {
                        let current_theme = current_theme.clone();
                        Callback::from(move |_: MouseEvent| {
                            set_html_theme(&theme_name);
                            current_theme.set(theme_name.clone());

                            if let Some(window) = window() {
                                if let Ok(Some(storage)) = window.local_storage() {
                                    let _ = storage.set_item("theme", &theme_name);
                                }
                            }
                        })
                    };

                    html! {
                        <li>
                            <a
                                onclick={on_click}
                                class={classes!(is_active.then(|| "active"))}
                            >
                                {label}
                            </a>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

fn set_html_theme(theme: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", theme);
            }
        }
    }
}

fn themes() -> Vec<(&'static str, &'static str)> {
    vec![
        ("dark", "dark"),
        ("cupcake", "cupcake"),
        ("bumblebee", "bumblebee"),
        ("emerald", "emerald"),
        ("corporate", "corporate"),
        ("synthwave-violet", "synthwave-violet"),
        ("synthwave", "synthwave"),
        ("retro", "retro"),
        ("cyberpunk", "cyberpunk"),
        ("valentine", "valentine"),
        ("halloween", "halloween"),
        ("garden", "garden"),
        ("forest", "FOREST"),
        ("aqua", "aqua"),
        ("lofi", "lofi"),
        ("pastel", "pastel"),
        ("fantasy", "fantasy"),
        ("wireframe", "wireframe"),
        ("black", "black"),
        ("luxury", "luxury"),
        ("dracula", "dracula"),
        ("cmyk", "cmyk"),
        ("autumn", "autumn"),
        ("business", "business"),
        ("acid", "acid"),
        ("lemonade", "lemonade"),
        ("night", "night"),
        ("coffee", "coffee"),
        ("winter", "winter"),
    ]
}

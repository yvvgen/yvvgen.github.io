use crate::data::profile_data::PROFILE;
use yew::{function_component, html, Html};

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav
            class="navbar bg-base-200/50 backdrop-blur-lg sticky top-0 z-50 border-b border-primary/20 shadow-lg"
        >
            <div class="container mx-auto px-4">
                <div class="navbar-start">
                    <a
                        href="#"
                        class="btn btn-ghost text-4xl font-display text-neon-primary transition duration-300 hover:text-secondary"
                    >
                        { PROFILE.name }
                    </a>
                </div>
                <div class="navbar-end">
                    <ul class="menu menu-horizontal p-0 hidden md:flex space-x-2">
                        <li>
                            <a
                                href="#experience"
                                class="btn btn-ghost text-lg hover:glow-primary transition duration-200"
                            >
                                { "Experiences" }
                            </a>
                        </li>
                        <li>
                            <a
                                href="#projects"
                                class="btn btn-ghost text-lg hover:glow-primary transition duration-200"
                            >
                                { "Projects" }
                            </a>
                        </li>
                        <li>
                            <a
                                href="#contact"
                                class="btn btn-ghost text-lg hover:glow-primary transition duration-200"
                            >
                                { "Contact" }
                            </a>
                        </li>
                    </ul>
                    // Mobile Dropdown Menu
                    <div class="dropdown dropdown-end md:hidden">
                        <div tabindex="0" role="button" class="btn btn-ghost">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h8m-8 6h16"
                                />
                            </svg>
                        </div>
                        <ul
                            tabindex="0"
                            class="menu menu-sm dropdown-content mt-3 z-[1] p-2 bg-base-200/50 backdrop-blur-lg rounded-box w-52"
                        >
                            <li>
                                <a href="#about" class="text-lg hover:text-neon-primary">
                                    { "About" }
                                </a>
                            </li>
                            <li>
                                <a href="#projects" class="text-lg hover:text-neon-primary">
                                    { "Projects" }
                                </a>
                            </li>
                            <li>
                                <a href="#skills" class="text-lg hover:text-neon-primary">
                                    { "Skills" }
                                </a>
                            </li>
                            <li>
                                <a href="#contact" class="text-lg hover:text-neon-primary">
                                    { "Contact" }
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </nav>
    }
}

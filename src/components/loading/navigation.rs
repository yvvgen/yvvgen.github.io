use yew::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav class="navbar bg-base-200/50 backdrop-blur-lg sticky top-0 z-50 border-b border-primary/20 shadow-lg">
            <div class="container mx-auto px-4">
                <div class="navbar-start">
                    <div class="h-8 w-32 bg-base-300/50 rounded animate-pulse"></div>
                </div>
                <div class="navbar-end">
                    <ul class="menu menu-horizontal p-0 hidden md:flex space-x-2">
                        <li><div class="h-8 w-24 bg-base-300/50 rounded animate-pulse"></div></li>
                        <li><div class="h-8 w-24 bg-base-300/50 rounded animate-pulse"></div></li>
                        <li><div class="h-8 w-24 bg-base-300/50 rounded animate-pulse"></div></li>
                    </ul>
                    <div class="dropdown dropdown-end md:hidden">
                        <div class="h-8 w-8 bg-base-300/50 rounded animate-pulse"></div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

// src/components/hero.rs
use yew::prelude::*;
use crate::components::ThemeSwitcher;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="min-h-screen flex flex-col items-center justify-center text-center px-4 bg-base-100 relative">
            <div class="absolute top-4 right-4 z-10">
                <ThemeSwitcher />
            </div>
            
            <div class="max-w-4xl space-y-8">
                <h1 class="text-6xl font-bold text-primary">
                    {"$ whoami"}
                </h1>
                
                // Test badges - should change colors with theme
                <div class="flex gap-2 justify-center flex-wrap">
                    <div class="badge badge-primary">{"Primary"}</div>
                    <div class="badge badge-secondary">{"Secondary"}</div>
                    <div class="badge badge-accent">{"Accent"}</div>
                    <div class="badge badge-info">{"Info"}</div>
                    <div class="badge badge-success">{"Success"}</div>
                    <div class="badge badge-warning">{"Warning"}</div>
                    <div class="badge badge-error">{"Error"}</div>
                </div>
                
                // Test buttons - should change with theme
                <div class="flex gap-2 justify-center flex-wrap">
                    <button class="btn btn-primary">{"Primary"}</button>
                    <button class="btn btn-secondary">{"Secondary"}</button>
                    <button class="btn btn-accent">{"Accent"}</button>
                </div>
                
                // Current theme indicator
                <div class="alert alert-info">
                    <span>{"Change theme in top-right to see colors change!"}</span>
                </div>
            </div>
        </section>
    }
}

use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="py-20 min-h-screen flex items-center justify-center">
            <div class="container mx-auto px-4">
                <div class="card card-synthwave lg:card-side scanlines animate-pulse">
                    <div class="card-body md:w-full">
                        <div class="h-10 w-48 bg-base-300/50 rounded mb-4"></div> // Title
                        <div class="space-y-2 text-left mb-6">
                            <div class="h-8 w-64 bg-base-300/50 rounded"></div> // Identity Name
                            <div class="h-6 w-48 bg-base-300/50 rounded"></div> // Identity Title
                            <div class="h-4 w-32 bg-base-300/50 rounded"></div> // Identity Location
                        </div>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
                            <div>
                                <div class="h-6 w-24 bg-base-300/50 rounded mb-3"></div> // Contact Title
                                <div class="h-4 w-40 bg-base-300/50 rounded mb-2"></div> // Phone
                                <div class="h-4 w-48 bg-base-300/50 rounded mb-4"></div> // Email
                            </div>
                            <div>
                                <div class="h-6 w-24 bg-base-300/50 rounded mb-3"></div> // Education Title
                                <ul class="space-y-2">
                                    <div class="h-4 w-full bg-base-300/50 rounded"></div>
                                    <div class="h-4 w-5/6 bg-base-300/50 rounded"></div>
                                    <div class="h-4 w-full bg-base-300/50 rounded"></div>
                                </ul>
                            </div>
                            <div class="md:col-span-2">
                                <div class="h-6 w-24 bg-base-300/50 rounded mb-3"></div> // Languages Title
                                <ul class="flex flex-wrap gap-2">
                                    <div class="h-8 w-20 bg-base-300/50 rounded-full"></div>
                                    <div class="h-8 w-24 bg-base-300/50 rounded-full"></div>
                                    <div class="h-8 w-16 bg-base-300/50 rounded-full"></div>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

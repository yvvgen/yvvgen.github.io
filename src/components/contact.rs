use crate::data::profile_data::PROFILE;
use yew::{function_component, html, Html};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="py-20 text-base-content">
            <h2 class="text-5xl font-extrabold text-center mb-12 text-neon-primary">
                { "Get In Touch" }
            </h2>
            <div class="container mx-auto px-6 max-w-xl">
                <div class="card-synthwave shadow-2xl p-8">
                    <p class="text-center mb-6 text-lg">
                        { format!("I'm currently focused on projects related to {}. Let's discuss collaboration.", PROFILE.interest) }
                    </p>
                    <form class="space-y-4">
                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{ "Your Name" }</span>
                            </div>
                            <input
                                type="text"
                                placeholder="Name"
                                class="input input-bordered w-full"
                            />
                        </label>
                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{ "Your Email" }</span>
                            </div>
                            <input
                                type="email"
                                placeholder="Email"
                                class="input input-bordered w-full"
                            />
                        </label>
                        <label class="form-control w-full">
                            <div class="label">
                                <span class="label-text">{ "Your Message" }</span>
                            </div>
                            <textarea
                                placeholder="Message"
                                rows="4"
                                class="textarea textarea-bordered h-24"
                            />
                        </label>
                        <button
                            type="submit"
                            class="btn btn-primary w-full shadow-lg transition duration-300 transform hover:scale-[1.01] hover:glow-primary"
                        >
                            { "Send Message" }
                        </button>
                    </form>
                </div>
            </div>
        </section>
    }
}

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalFrameProps {
    #[prop_or_default]
    pub children: Children,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(ModalFrame)]
pub fn modal_frame(props: &ModalFrameProps) -> Html {
    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div class="bg-base-200 p-4 rounded-lg shadow-lg max-w-3xl w-full relative">
                <button
                    class="absolute top-2 right-2 btn btn-sm btn-circle btn-ghost"
                    onclick={props.on_close.clone()}
                >
                    {"✕"}
                </button>
                { for props.children.iter() }
            </div>
        </div>
    }
}

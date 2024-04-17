use leptos::{*, html::Input};

use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black";

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! { cx,
        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto border border-gray-300 rounded p-5 bg-gray-100 " node_ref=chat_div_ref>
                        {move || conversation.get().messages.iter().map(move |message| {
                            let class_str = if message.user { USER_MESSAGE_CLASS } eles { MODEL_MESSAGE_CLASS };
                            view! { cx,
                                <div>
                                    {message.text.clone()}
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }
        </div>
    }
}
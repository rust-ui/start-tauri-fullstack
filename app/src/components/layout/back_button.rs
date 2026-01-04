use icons::ChevronLeft;
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn BackButton() -> impl IntoView {
    let go_back = move |_| {
        let window = window();
        if let Ok(history) = window.history() {
            let _ = history.back();
        }
    };

    view! {
        <Button
            variant=ButtonVariant::Ghost
            size=ButtonSize::Icon
            class="rounded-full bg-muted size-8"
            on:click=go_back
        >
            <ChevronLeft class="size-5" />
        </Button>
    }
}

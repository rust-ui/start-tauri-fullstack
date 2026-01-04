use icons::RefreshCw;
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

/// Utility button to force reload the page positioned absolute top-right.
/// Only visible for iOS dev.
#[component]
pub fn ReloadButton() -> impl IntoView {
    view! {
        <Button
            variant=ButtonVariant::Ghost
            size=ButtonSize::Icon
            class="hidden transition-transform active:scale-95 supports-[-webkit-touch-callout:none]:block"
            on:click=|_| {
                let window = leptos::prelude::window();
                let _ = window.location().reload();
            }
        >
            <RefreshCw class="size-5 text-muted-foreground" />
        </Button>
    }
}

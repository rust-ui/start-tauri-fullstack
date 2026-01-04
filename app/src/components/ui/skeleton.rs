use leptos::prelude::*;
use leptos_ui::void;

const PULSE_ANIMATION: &str = "animate-pulse";

mod components {
    use super::*;
    void! {Skeleton, div, PULSE_ANIMATION, "rounded-md bg-muted"}
}

pub use components::*;

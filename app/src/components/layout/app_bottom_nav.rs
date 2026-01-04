use icons::{House, Scroll, Settings};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::hooks::use_is_current_path::use_is_current_path;
use crate::components::ui::bottom_nav::{
    BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel,
};
use crate::domain::settings::routes::SettingsRoutes;
use crate::domain::template::routing::TemplateRoutes;
use crate::domain::home::HomeRoutes;

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let navigate = use_navigate();
    let is_current_path = use_is_current_path();

    let nav_items: [(&'static str, &'static str, AnyView); 3] = [
        (HomeRoutes::base_url(), HomeRoutes::label(), view! { <House /> }.into_any()),
        (TemplateRoutes::base_url(), TemplateRoutes::label(), view! { <Scroll /> }.into_any()),
        (SettingsRoutes::base_url(), SettingsRoutes::label(), view! { <Settings /> }.into_any()),
    ];

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 sm:hidden">
            <BottomNavGrid>
                {nav_items
                    .into_iter()
                    .map(|(path, label, icon)| {
                        let navigate = navigate.clone();
                        let is_current_path = is_current_path.clone();
                        view! {
                            <BottomNavButton
                                on:click=move |_| {
                                    navigate(path, Default::default());
                                }
                                attr:aria-current=move || is_current_path(path)
                            >
                                {icon}
                                <BottomNavLabel>{label}</BottomNavLabel>
                            </BottomNavButton>
                        }
                    })
                    .collect_view()}
            </BottomNavGrid>
        </BottomNav>
    }
}

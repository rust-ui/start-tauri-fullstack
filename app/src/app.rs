use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::layout::app_bottom_nav::AppBottomNav;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::components::layout::header::Header;
use crate::domain::home::{HomePage, HomeRoutes};
use crate::domain::settings::page_settings::SettingsPage;
use crate::domain::settings::routes::SettingsRoutes;
use crate::domain::template::routing::{PageTemplateDetails, TemplatePage, TemplateRoutes};
use crate::utils::param::PARAM;

#[component]
pub fn App() -> impl IntoView {
    let theme_mode = ThemeMode::init();

    provide_meta_context();

    view! {
        <Title text="Rust/UI Starters — Cross-Platform Apps" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <AppWrapper>
                <Header />

                <main class="overflow-y-auto flex-1 overflow-x-clip">
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=StaticSegment(HomeRoutes::base_url()) view=HomePage />
                        <Route path=StaticSegment(SettingsRoutes::base_segment()) view=SettingsPage />
                        // Templates
                        <ParentRoute path=StaticSegment(TemplateRoutes::base_segment()) view=Outlet>
                            <Route path=StaticSegment("") view=TemplatePage />
                            <Route path=ParamSegment(PARAM::UNID) view=PageTemplateDetails />
                        </ParentRoute>
                    </Routes>
                </main>
            </AppWrapper>

            <AppBottomNav />
        </Router>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <p>"Not Found."</p> }
}

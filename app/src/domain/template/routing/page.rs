use leptos::prelude::*;

use crate::domain::template::components::template_list::TemplateList;

#[component]
pub fn TemplatePage() -> impl IntoView {
    view! {
        <div class="flex relative flex-col mx-auto w-full max-w-md h-full">
            // Header
            <header class="px-4 pt-4 pb-3">
                <h1 class="text-2xl font-bold tracking-tight">"My Templates"</h1>
                <p class="text-sm text-muted-foreground">"Manage your templates"</p>
            </header>

            <TemplateList />
        </div>
    }
}

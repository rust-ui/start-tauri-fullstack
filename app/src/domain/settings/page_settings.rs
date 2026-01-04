use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 p-2">
            <h1 class="text-2xl font-bold">"Settings"</h1>
            <p class="text-muted-foreground">"This is the settings page."</p>
        </div>
    }
}

use icons::Plus;
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize};
use crate::components::ui::input::Input;
use crate::domain::template::template_db::Template;
use crate::domain::template::template_services::AddTemplate;

#[component]
pub fn TemplateForm(add_template: ServerMultiAction<AddTemplate>) -> impl IntoView {
    view! {
        <MultiActionForm action=add_template>
            <footer class="flex gap-2 items-center p-4 border-t bg-background">
                <Input attr:name=Template::FIELD_NAMES.title attr:placeholder="Title..." class="flex-1" />
                <Input attr:name=Template::FIELD_NAMES.description attr:placeholder="Description..." class="flex-1" />

                <Button attr:r#type="submit" size=ButtonSize::Icon>
                    <Plus class="size-5" />
                </Button>
            </footer>
        </MultiActionForm>
    }
}

use icons::{CircleCheckBig, ClipboardList, Trash2};
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::card::{Card, CardDescription};
use crate::components::ui::empty::{
    Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use crate::components::ui::pressable::Pressable;
use crate::components::ui::skeleton::Skeleton;
use crate::domain::template::components::template_form::TemplateForm;
use crate::domain::template::routing::TemplateRoutes;
use crate::domain::template::template_db::Template;
use crate::domain::template::template_services::{AddTemplate, DeleteTemplate, get_templates};

#[component]
pub fn TemplateList() -> impl IntoView {
    let add_template = ServerMultiAction::<AddTemplate>::new();
    let delete_template = ServerAction::<DeleteTemplate>::new();

    let templates = Resource::new(
        move || (add_template.version().get(), delete_template.version().get()),
        move |_| get_templates(),
    );

    view! {
        <div class="overflow-y-auto flex-1 px-4">
            <Transition fallback=|| {
                view! { <TemplateListSkeletonCards count=3 /> }
            }>
                {move || {
                    templates
                        .and_then(|templates| {
                            if templates.is_empty() {
                                view! { <TemplateListEmptyState /> }.into_any()
                            } else {
                                view! {
                                    <div class="flex flex-col gap-2">
                                        {templates
                                            .iter()
                                            .map(|template| {
                                                view! {
                                                    <TemplateListCard
                                                        template=template.clone()
                                                        delete_template=delete_template
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Transition>
        </div>

        // Input form at bottom
        <TemplateForm add_template=add_template />
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn TemplateListCard(
    template: Template,
    delete_template: ServerAction<DeleteTemplate>,
) -> impl IntoView {
    let unid = template.unid;
    let href = TemplateRoutes::details_url(unid);

    let on_delete = move |ev: SubmitEvent| {
        ev.prevent_default();
        delete_template.dispatch(DeleteTemplate { unid });
    };

    let stop_propagation = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <Pressable>
            <A href=href attr:class="block">
                <Card class="flex-row gap-3 items-center p-4 hover:bg-accent/50">
                    <CircleCheckBig class="shrink-0 size-5 text-primary/40" />
                    <div class="flex flex-col flex-1 gap-1">
                        <span class="font-medium">{template.title.clone()}</span>
                        <CardDescription class="line-clamp-1">{template.description.clone()}</CardDescription>
                    </div>
                    <form on:submit=on_delete on:click=stop_propagation>
                        <Button
                            attr:r#type="submit"
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            class="size-8 text-muted-foreground hover:text-destructive"
                        >
                            <Trash2 class="size-4" />
                        </Button>
                    </form>
                </Card>
            </A>
        </Pressable>
    }
}

/* ========================================================== */
/*                       ✨ STATE ✨.                         */
/* ========================================================== */

#[component]
fn TemplateListEmptyState() -> impl IntoView {
    view! {
        <Empty class="border-none">
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <ClipboardList />
                </EmptyMedia>
                <EmptyTitle>"No templates yet"</EmptyTitle>
                <EmptyDescription>"Add your first template below to get started"</EmptyDescription>
            </EmptyHeader>
        </Empty>
    }
}

#[component]
fn TemplateListSkeletonCards(#[prop()] count: usize) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            {(0..count)
                .map(|_| {
                    view! {
                        <Card class="flex-row gap-3 items-center p-4 py-4">
                            <Skeleton class="rounded-full size-5 shrink-0" />
                            <Skeleton class="flex-1 h-4" />
                            <Skeleton class="rounded-md size-8 shrink-0" />
                        </Card>
                    }
                })
                .collect_view()}
        </div>
    }
}

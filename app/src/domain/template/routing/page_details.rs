use icons::FileText;
use leptos::prelude::*;

use crate::components::layout::back_button::BackButton;
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::empty::{
    Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use crate::components::ui::skeleton::Skeleton;
use crate::domain::template::template_db::Template;
use crate::domain::template::template_services::get_template_by_unid;
use crate::utils::param::UnidParam;

#[component]
pub fn PageTemplateDetails() -> impl IntoView {
    let unid_param = UnidParam::extract_unid();

    let template_resource = Resource::new(move || unid_param.get(), get_template_by_unid);

    view! {
        <div class="flex flex-col mx-auto w-full max-w-md h-full bg-background">
            // Header with back button
            <header class="flex items-center px-4 pt-4 pb-3">
                <BackButton />
                <h1 class="flex-1 text-lg font-semibold text-center">"Template Details"</h1>
                // Spacer to balance the back button
                <div class="size-8" />
            </header>

            // Content
            <div class="overflow-y-auto flex-1 px-4 pb-4">
                <Transition fallback=|| {
                    view! { <TemplateDetailSkeleton /> }
                }>
                    {move || {
                        template_resource
                            .and_then(|template| {
                                match template {
                                    Some(template) => {
                                        view! { <TemplateDetailContent template=template.clone() /> }.into_any()
                                    }
                                    None => view! { <TemplateNotFound /> }.into_any(),
                                }
                            })
                    }}
                </Transition>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn TemplateDetailContent(template: Template) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            // Icon header
            <div class="flex justify-center py-6">
                <div class="flex justify-center items-center rounded-full size-20 bg-primary/10">
                    <FileText class="size-10 text-primary" />
                </div>
            </div>

            // Title
            <h2 class="text-2xl font-bold text-center">{template.title.clone()}</h2>

            // Description card
            <Card>
                <CardHeader>
                    <CardTitle class="text-sm text-muted-foreground">"Description"</CardTitle>
                </CardHeader>
                <CardContent>
                    <p class="leading-relaxed">{template.description.clone()}</p>
                </CardContent>
            </Card>

            // Metadata
            <Card>
                <CardHeader>
                    <CardTitle class="text-sm text-muted-foreground">"Details"</CardTitle>
                </CardHeader>
                <CardContent>
                    <dl class="grid gap-y-2 gap-x-4 text-sm grid-cols-[auto_1fr]">
                        <dt class="text-muted-foreground">"ID"</dt>
                        <dd class="font-mono text-xs">{template.unid.to_string()}</dd>

                        <dt class="text-muted-foreground">"Created"</dt>
                        <dd>{template.created_at.date().to_string()}</dd>

                        <dt class="text-muted-foreground">"Updated"</dt>
                        <dd>{template.updated_at.date().to_string()}</dd>
                    </dl>
                </CardContent>
            </Card>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn TemplateNotFound() -> impl IntoView {
    view! {
        <Empty class="border-none">
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <FileText />
                </EmptyMedia>
                <EmptyTitle>"Template not found"</EmptyTitle>
                <EmptyDescription>"This template doesn't exist or has been removed"</EmptyDescription>
            </EmptyHeader>
        </Empty>
    }
}

#[component]
fn TemplateDetailSkeleton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            // Icon placeholder
            <div class="flex justify-center py-6">
                <Skeleton class="rounded-full size-20" />
            </div>

            // Title placeholder
            <Skeleton class="mx-auto w-48 h-8" />

            // Description card placeholder
            <Card>
                <CardHeader>
                    <Skeleton class="w-24 h-4" />
                </CardHeader>
                <CardContent>
                    <Skeleton class="w-full h-4" />
                    <Skeleton class="mt-2 w-3/4 h-4" />
                </CardContent>
            </Card>

            // Metadata card placeholder
            <Card>
                <CardHeader>
                    <Skeleton class="w-16 h-4" />
                </CardHeader>
                <CardContent>
                    <div class="grid gap-y-2 gap-x-4 grid-cols-[auto_1fr]">
                        <Skeleton class="w-8 h-4" />
                        <Skeleton class="w-48 h-4" />
                        <Skeleton class="w-16 h-4" />
                        <Skeleton class="w-24 h-4" />
                        <Skeleton class="w-16 h-4" />
                        <Skeleton class="w-24 h-4" />
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}

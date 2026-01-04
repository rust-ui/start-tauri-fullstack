use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json, PostUrl};
use uuid::Uuid;

use crate::domain::template::template_db::Template;

#[server(GetTemplates, "/api", input = GetUrl, output = Json)]
pub async fn get_templates() -> Result<Vec<Template>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;

    let templates = Template::get_all(&app_state.pool).await.map_err(ServerFnError::new)?;

    Ok(templates)
}

#[server(AddTemplate, "/api", input = PostUrl, output = Json)]
pub async fn add_template(title: String, description: String) -> Result<Template, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;

    let template = Template::add(&app_state.pool, title, description)
        .await
        .map_err(ServerFnError::new)?;

    Ok(template)
}

#[server(GetTemplateByUnid, "/api", input = GetUrl, output = Json)]
pub async fn get_template_by_unid(unid: Uuid) -> Result<Option<Template>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;

    let template = Template::get_by_unid(&app_state.pool, unid)
        .await
        .map_err(ServerFnError::new)?;

    Ok(template)
}

#[server(DeleteTemplate, "/api", input = GetUrl, output = Json)]
pub async fn delete_template(unid: Uuid) -> Result<Uuid, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;

    let template_unid =
        Template::delete(&app_state.pool, unid).await.map_err(ServerFnError::new)?;

    Ok(template_unid)
}

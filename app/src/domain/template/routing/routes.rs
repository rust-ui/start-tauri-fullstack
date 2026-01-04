use uuid::Uuid;

pub struct TemplateRoutes;

impl TemplateRoutes {
    pub fn base_segment() -> &'static str {
        "templates"
    }

    pub fn base_url() -> &'static str {
        "/templates"
    }

    pub fn label() -> &'static str {
        "Templates"
    }

    pub fn details_url(unid: Uuid) -> String {
        format!("/templates/{unid}")
    }
}

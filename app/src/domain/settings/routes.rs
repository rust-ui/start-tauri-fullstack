pub struct SettingsRoutes;

impl SettingsRoutes {
    pub fn base_segment() -> &'static str {
        "settings"
    }

    pub fn base_url() -> &'static str {
        "/settings"
    }

    pub fn label() -> &'static str {
        "Settings"
    }
}

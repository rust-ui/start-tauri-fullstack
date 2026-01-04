use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use uuid::Uuid;

pub struct PARAM;

impl PARAM {
    pub const UNID: &str = "unid";
}

#[derive(Params, PartialEq, Clone)]
pub struct UnidParam {
    pub unid: String,
}

impl UnidParam {
    pub fn extract_unid() -> Memo<Uuid> {
        let params = use_params::<Self>();
        Memo::new(move |_| {
            params
                .read()
                .as_ref()
                .map(|params| Uuid::parse_str(&params.unid).unwrap_or_default())
                .unwrap_or_default()
        })
    }
}

use axum::extract::FromRef;
use leptos::prelude::{LeptosOptions, *};
#[cfg(feature = "ssr")]
use sqlx::postgres::PgPool;

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    #[cfg(feature = "ssr")]
    pub pool: PgPool,
}

pub fn use_app_state() -> Result<AppState, ServerFnError> {
    use_context::<AppState>().ok_or_else(|| ServerFnError::ServerError("App state missing.".into()))
}

use app::app::App;
use app::common::app_state::AppState;
use app::shell::shell;
use axum::Router;
use axum::body::Body as AxumBody;
use axum::extract::State;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use leptos::prelude::*;
use leptos_axum::{
    LeptosRoutes, generate_route_list, handle_server_fns_with_context,
    render_app_to_stream_with_context,
};
use sqlx::PgPool;

use crate::fallback::file_and_error_handler;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub async fn build_app_router(conf_file: ConfFile, pool: PgPool) -> anyhow::Result<Router> {
    let leptos_options = conf_file.leptos_options;

    let routes = generate_route_list(|| view! { <App /> });

    let app_state = AppState { leptos_options, pool: pool.clone() };

    Ok(Router::new()
        .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
        // .layer(PropertyAccessLayer::new()) // custom middleware for properties
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state))
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[axum_macros::debug_handler]
pub async fn server_fn_handler(
    State(state): State<AppState>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(state.clone());
        },
        request,
    )
    .await
}

#[axum_macros::debug_handler]
pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let leptos_options = app_state.leptos_options.clone();

    let handler = render_app_to_stream_with_context(
        move || {
            provide_context(app_state.clone());
        },
        move || shell(leptos_options.clone()),
    );
    handler(req).await.into_response()
}

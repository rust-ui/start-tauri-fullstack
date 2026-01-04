use http::status::StatusCode;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Internal Server Error")]
    InternalServerError,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Renders errors and sets HTTP status code on SSR. Used in `server/src/fallback.rs`.
#[component]
pub fn ErrorBoundary(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors_signal: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors_signal = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors_signal {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    let errors = errors_signal.get();

    let errors: Vec<AppError> =
        errors.into_iter().filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned()).collect();
    println!("Errors: {errors:#?}");

    #[cfg(feature = "ssr")]
    let response = use_context::<ResponseOptions>();
    #[cfg(feature = "ssr")]
    if let Some(response) = response
        && !errors.is_empty()
    {
        response.set_status(errors[0].status_code());
    }

    view! {
        <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
        <For
            each=move || { errors.clone().into_iter().enumerate() }
            key=|(index, _error)| *index
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}

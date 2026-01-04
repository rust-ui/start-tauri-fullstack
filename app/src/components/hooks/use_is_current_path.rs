use leptos::prelude::Get;
use leptos_router::hooks::use_location;

/// Returns a closure that checks if a given path matches the current location.
///
/// For the root path "/", it checks for exact match.
/// For other paths, it checks if the current pathname starts with the given path.
pub fn use_is_current_path() -> impl Fn(&'static str) -> &'static str + Clone {
    let location = use_location();

    move |path: &'static str| -> &'static str {
        let pathname = location.pathname.get();
        let matches = if path == "/" { pathname == "/" } else { pathname.starts_with(path) };
        if matches { "page" } else { "" }
    }
}

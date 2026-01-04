use leptos::prelude::*;
use web_sys::Storage;

#[derive(Debug, Clone, Copy)]
pub struct ThemeMode {
    state_signal: RwSignal<bool>,
}

const LOCALSTORAGE_KEY: &str = "darkmode";

/// Hook to access the dark mode context
pub fn use_theme_mode() -> ThemeMode {
    expect_context::<ThemeMode>()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

impl ThemeMode {
    #[must_use]
    /// Initializes a new ThemeMode instance.
    pub fn init() -> Self {
        let theme_mode = Self { state_signal: RwSignal::new(false) };

        provide_context(theme_mode);

        // Use Effect to handle browser-only initialization
        Effect::new(move |_| {
            let initial = Self::get_storage_state().unwrap_or(Self::prefers_dark_mode());
            theme_mode.state_signal.set(initial);
        });

        theme_mode
    }

    pub fn toggle(&self) {
        self.state_signal.update(|state| {
            *state = !*state;
            Self::set_storage_state(*state);
        });
    }

    #[must_use]
    pub fn get(&self) -> bool {
        self.state_signal.get()
    }

    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.state_signal.get()
    }

    /* ========================================================== */
    /*                     ✨ FUNCTIONS ✨                        */
    /* ========================================================== */

    fn get_storage() -> Option<Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    fn get_storage_state() -> Option<bool> {
        Self::get_storage()
            .and_then(|storage| storage.get(LOCALSTORAGE_KEY).ok())
            .flatten()
            .and_then(|entry| entry.parse::<bool>().ok())
    }

    fn prefers_dark_mode() -> bool {
        web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            .map(|media| media.matches())
            .unwrap_or_default()
    }

    fn set_storage_state(state: bool) {
        if let Some(storage) = Self::get_storage() {
            storage.set(LOCALSTORAGE_KEY, state.to_string().as_str()).ok();
        }
    }
}

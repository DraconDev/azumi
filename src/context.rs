use std::cell::RefCell;
use std::future::Future;

tokio::task_local! {
    static CURRENT_PATH: String;
}

/// Run a future with the current request path derived from context
pub async fn with_path<F: Future>(path: String, f: F) -> F::Output {
    CURRENT_PATH.scope(path, f).await
}

/// Get the current request path if available in the task context
pub fn get_current_path() -> Option<String> {
    CURRENT_PATH.try_with(|p| p.clone()).ok()
}

// ============================================================================
// Sync Context for Rendering (Thread Local)
// ============================================================================

#[derive(Clone, Default, Debug)]
pub struct PageMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

std::thread_local! {
    static PAGE_META: RefCell<PageMeta> = RefCell::new(PageMeta::default());
}

/// Set the metadata for the current page render.
/// This should be called by the #[azumi::page] wrapper.
pub fn set_page_meta(title: Option<String>, description: Option<String>, image: Option<String>) {
    PAGE_META.with(|params| {
        *params.borrow_mut() = PageMeta {
            title,
            description,
            image,
        };
    });
}

/// Get the current page metadata.
/// This is used by the `head!` macro or layout components.
pub fn get_page_meta() -> PageMeta {
    PAGE_META.with(|params| params.borrow().clone())
}

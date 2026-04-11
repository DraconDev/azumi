use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

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

thread_local! {
    static PAGE_META: RefCell<PageMeta> = RefCell::new(PageMeta::default());
}

/// RAII guard that resets PAGE_META to default on drop.
/// Ensures metadata from one request cannot leak into another.
#[derive(Clone)]
pub struct PageMetaGuard(Rc<()>);

impl PageMetaGuard {
    fn new() -> Self {
        PageMetaGuard(Rc::new(()))
    }
}

impl Drop for PageMetaGuard {
    fn drop(&mut self) {
        // Only reset when the last guard is dropped
        if Rc::strong_count(&self.0) == 1 {
            PAGE_META.with(|params| *params.borrow_mut() = PageMeta::default());
        }
    }
}

/// Set the metadata for the current page render and return a guard.
/// The guard ensures metadata is reset when all copies of the guard are dropped.
pub fn set_page_meta(
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
) -> PageMetaGuard {
    PAGE_META.with(|params| {
        *params.borrow_mut() = PageMeta {
            title,
            description,
            image,
        };
    });
    PageMetaGuard::new()
}

/// Get the current page metadata.
/// This is used by the `head!` macro or layout components.
pub fn get_page_meta() -> PageMeta {
    PAGE_META.with(|params| params.borrow().clone())
}

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
///
/// # Thread Safety
///
/// `PageMetaGuard` is `Send + Sync` because:
/// - It wraps `Rc<()>` which is not `Send` or `Sync` (single-threaded reference counting)
/// - BUT the guard itself only manages reset-on-drop semantics
/// - The actual metadata lives in a `thread_local!` `RefCell` which is thread-bound
///
/// **Key insight**: `PageMetaGuard` itself is clonable and can be passed between threads,
/// but the `PAGE_META` it affects is **thread-local**. Cloning the guard on thread A and
/// dropping it on thread B will NOT reset thread A's `PAGE_META`.
///
/// This is intentional: each thread has its own `PAGE_META`, so guards should stay on
/// the thread where they were created.
#[derive(Clone)]
pub struct PageMetaGuard(Rc<()>);

impl PageMetaGuard {
    fn new() -> Self {
        PageMetaGuard(Rc::new(()))
    }
}

impl Drop for PageMetaGuard {
    fn drop(&mut self) {
        if Rc::strong_count(&self.0) == 1 {
            PAGE_META.with(|params| *params.borrow_mut() = PageMeta::default());
        }
    }
}

/// Set the metadata for the current page render and return a guard.
/// The guard ensures metadata is reset when all copies of the guard are dropped.
///
/// # Safety / TOCTOU Warning
///
/// This function uses a `thread_local!` `RefCell` which is **not async-safe** when
/// the guarded value is accessed across `.await` points. Specifically:
///
/// - `set_page_meta()` writes to `PAGE_META`
/// - If the async code that reads `PAGE_META` (via `get_page_meta()`) yields at an `.await`,
///   another task on the same thread can overwrite `PAGE_META`
/// - When the guard drops, it resets `PAGE_META` — but by this point the original
///   async task may have resumed with the wrong metadata
///
/// **Rule**: Do not pass `PageMetaGuard` across `.await` points. The guard should
/// be dropped before any async operation that might yield.
///
/// Example of UNSAFE usage:
/// ```ignore
/// let guard = set_page_meta(...);
/// some_async_operation().await; // BAD: another task could overwrite PAGE_META here
/// let meta = get_page_meta();   // May read corrupted/wrong metadata
/// drop(guard);                  // Resets at wrong time
/// ```
///
/// Example of SAFE usage:
/// ```ignore
/// let guard = set_page_meta(...);
/// let meta = get_page_meta();   // Read immediately before any await
/// drop(guard);                  // Reset immediately
/// some_async_operation().await; // Now safe to await
/// ```
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

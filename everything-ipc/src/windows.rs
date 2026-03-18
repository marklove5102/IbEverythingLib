use windows::{Win32::Foundation::HANDLE, core::Free};

/// A Windows handle wrapper
///
/// # Safety
///
/// This type automatically closes the handle when dropped.
#[derive(Debug)]
pub struct Handle {
    inner: HANDLE,
}

impl Handle {
    pub fn new(handle: HANDLE) -> Self {
        Handle { inner: handle }
    }

    pub fn get(&self) -> HANDLE {
        self.inner
    }

    pub fn is_null(&self) -> bool {
        self.inner.0.is_null()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { self.inner.free() };
    }
}

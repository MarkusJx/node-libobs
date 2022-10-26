use crate::obs::sys;
use crate::obs::util::napi_error::to_napi_error_string;
use std::sync::{Mutex, MutexGuard};

pub struct ObsGuard {
    library: Mutex<sys::Bindings>,
    shutdown: bool,
}

impl ObsGuard {
    pub fn new(library: sys::Bindings, shutdown: bool) -> Self {
        Self {
            library: Mutex::new(library),
            shutdown,
        }
    }

    pub(crate) fn library(&self) -> napi::Result<MutexGuard<sys::Bindings>> {
        self.library
            .lock()
            .map_err(|e| to_napi_error_string(e.to_string()))
    }
}

impl Drop for ObsGuard {
    fn drop(&mut self) {
        if self.shutdown {
            if let Ok(library) = self.library() {
                unsafe {
                    library.obs_shutdown();
                }
            }
        }
    }
}

use crate::obs::sys;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::traits::raw::Raw;
use crate::obs::util::obs_guard::ObsGuard;
use crate::obs::util::types::ResultType;
use std::ffi::CStr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

pub struct ObsData {
    data: AtomicPtr<sys::obs_data_t>,
    guard: Arc<ObsGuard>,
}

impl ObsData {
    pub fn to_json_string(&self) -> ResultType<String> {
        let json = unsafe { self.guard.library.obs_data_get_json(self.raw()) };

        if json.is_null() {
            Err("Failed to get json string".into())
        } else {
            let json = unsafe { CStr::from_ptr(json) };

            Ok(json.to_string_lossy().into_owned())
        }
    }

    pub fn library(&self) -> &sys::Bindings {
        &self.guard.library
    }
}

impl FromRaw<sys::obs_data_t> for ObsData {
    unsafe fn from_raw_unchecked(raw: *mut sys::obs_data_t, guard: Arc<ObsGuard>) -> ObsData {
        Self {
            data: AtomicPtr::new(raw),
            guard,
        }
    }
}

impl Raw<sys::obs_data_t> for ObsData {
    unsafe fn raw(&self) -> *mut sys::obs_data_t {
        self.data.load(Ordering::Relaxed)
    }
}

impl Drop for ObsData {
    fn drop(&mut self) {
        unsafe {
            self.guard.library.obs_data_release(self.raw());
        }
    }
}

unsafe impl Send for ObsData {}
unsafe impl Sync for ObsData {}

use crate::obs::sys;
use crate::obs::traits::from_raw::{FromRaw, Guard};
use crate::obs::traits::raw::Raw;
use crate::obs::util::types::ResultType;
use std::ffi::CStr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct ObsData {
    data: AtomicPtr<sys::obs_data_t>,
    _guard: Guard,
}

impl ObsData {
    pub fn to_json_string(&self) -> ResultType<String> {
        let json = unsafe { sys::obs_data_get_json(self.raw()) };

        if json.is_null() {
            Err("Failed to get json string".into())
        } else {
            let json = unsafe { CStr::from_ptr(json) };

            Ok(json.to_string_lossy().into_owned())
        }
    }
}

impl FromRaw<sys::obs_data_t> for ObsData {
    unsafe fn from_raw_unchecked(raw: *mut sys::obs_data_t, guard: Guard) -> ObsData {
        Self {
            data: AtomicPtr::new(raw),
            _guard: guard,
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
            sys::obs_data_release(self.raw());
        }
    }
}

unsafe impl Send for ObsData {}
unsafe impl Sync for ObsData {}

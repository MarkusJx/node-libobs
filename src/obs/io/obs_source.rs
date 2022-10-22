use crate::obs::data::obs_properties::ObsProperties;
use crate::obs::data::obs_settings::ObsSettings;
use crate::obs::sys;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::traits::raw::Raw;
use crate::obs::util::napi_error::to_napi_error_str;
use crate::obs::util::obs_guard::ObsGuard;
use core::ffi::CStr;
use std::ffi::CString;
use std::sync::Arc;

/// An obs source.
///
/// # Example
/// ```ts
/// const source = await obs.createSource('My Source', 'monitor_capture', new ObsSettings({
///    monitor: 1,
///    capture_cursor: false,
///    method: 2,
/// }));
/// ```
#[napi]
pub struct ObsSource {
    source: *mut sys::obs_source_t,
    id: String,
    guard: Arc<ObsGuard>,
}

#[napi]
impl ObsSource {
    /// Get the default settings for this source.
    #[napi]
    pub fn get_default_settings(&self) -> napi::Result<ObsSettings> {
        let id = CString::new(self.id.clone())?;
        let settings = unsafe { self.guard.library.obs_get_source_defaults(id.as_ptr()) };

        if settings.is_null() {
            Err(to_napi_error_str("Failed to get default settings"))
        } else {
            Ok(ObsSettings::from_raw(settings, self.guard.clone()))
        }
    }

    /// Get the properties for this source.
    #[napi]
    pub fn get_properties(&self) -> napi::Result<ObsProperties> {
        let properties = unsafe { self.guard.library.obs_source_properties(self.source) };

        if properties.is_null() {
            Err(to_napi_error_str("Failed to get properties"))
        } else {
            Ok(ObsProperties::from_raw(properties, self.guard.clone()))
        }
    }

    /// Update the settings of the source.
    #[napi]
    pub fn update_settings(&self, settings: &ObsSettings) {
        unsafe {
            self.guard
                .library
                .obs_source_update(self.source, settings.raw());
        }
    }

    /// Get the settings of the source.
    #[napi(getter)]
    pub fn get_settings(&self) -> napi::Result<ObsSettings> {
        let settings = unsafe { self.guard.library.obs_source_get_settings(self.source) };

        if settings.is_null() {
            Err(to_napi_error_str("Failed to get settings"))
        } else {
            Ok(ObsSettings::from_raw(settings, self.guard.clone()))
        }
    }
}

impl FromRaw<sys::obs_source_t> for ObsSource {
    unsafe fn from_raw_unchecked(source: *mut sys::obs_source_t, guard: Arc<ObsGuard>) -> Self {
        let id = {
            let id = guard.library.obs_source_get_id(source);
            let id = CStr::from_ptr(id as *mut _);
            id.to_string_lossy().to_string()
        };

        Self { source, id, guard }
    }
}

impl Raw<sys::obs_source_t> for ObsSource {
    unsafe fn raw(&self) -> *mut sys::obs_source_t {
        self.source
    }
}

unsafe impl Send for ObsSource {}

impl Drop for ObsSource {
    fn drop(&mut self) {
        unsafe {
            self.guard.library.obs_source_release(self.source);
        }
    }
}

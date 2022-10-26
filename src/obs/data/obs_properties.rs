use crate::obs::data::obs_property::ObsProperty;
use crate::obs::sys;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::util::obs_guard::ObsGuard;
use std::ffi::CString;
use std::sync::Arc;

/// An obs properties object.
#[napi]
pub struct ObsProperties {
    properties: *mut sys::obs_properties_t,
    guard: Arc<ObsGuard>,
}

#[napi]
impl ObsProperties {
    /// Get a property by its name.
    #[napi]
    pub fn get_property(&self, name: String) -> napi::Result<Option<ObsProperty>> {
        let name = CString::new(name)?;
        let property = unsafe {
            self.guard
                .library()?
                .obs_properties_get(self.properties, name.as_ptr())
        };

        if property.is_null() {
            Ok(None)
        } else {
            Ok(Some(ObsProperty::from_raw(property, self.guard.clone())))
        }
    }

    /// Get a list of all properties stored in this object.
    #[napi]
    pub fn list_properties(&self) -> napi::Result<Vec<ObsProperty>> {
        let library = self.guard.library()?;

        let mut properties = Vec::new();
        let mut property = unsafe { library.obs_properties_first(self.properties) };
        properties.push(ObsProperty::from_raw(property, self.guard.clone()));

        while unsafe { library.obs_property_next(&mut property) } && !property.is_null() {
            properties.push(ObsProperty::from_raw(property, self.guard.clone()));
        }

        Ok(properties)
    }
}

impl FromRaw<sys::obs_properties_t> for ObsProperties {
    unsafe fn from_raw_unchecked(
        properties: *mut sys::obs_properties_t,
        guard: Arc<ObsGuard>,
    ) -> Self {
        Self { properties, guard }
    }
}

impl Drop for ObsProperties {
    fn drop(&mut self) {
        if let Ok(library) = self.guard.library() {
            unsafe {
                //library.obs_properties_destroy(self.properties);
            }
        }
    }
}

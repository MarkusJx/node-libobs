use crate::obs::data::obs_property::ObsProperty;
use crate::obs::sys;
use crate::obs::traits::from_raw::{FromRaw, Guard};
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
    pub fn get_property(&self, name: String) -> Option<ObsProperty> {
        let name = CString::new(name).ok()?;
        let property = unsafe { sys::obs_properties_get(self.properties, name.as_ptr()) };

        if property.is_null() {
            None
        } else {
            Some(ObsProperty::from_raw(property, Some(self.guard.clone())))
        }
    }

    /// Get a list of all properties stored in this object.
    #[napi]
    pub fn list_properties(&self) -> Vec<ObsProperty> {
        let mut properties = Vec::new();
        let mut property = unsafe { sys::obs_properties_first(self.properties) };
        properties.push(ObsProperty::from_raw(property, Some(self.guard.clone())));

        while unsafe { sys::obs_property_next(&mut property) } && !property.is_null() {
            properties.push(ObsProperty::from_raw(property, Some(self.guard.clone())));
        }

        properties
    }
}

impl FromRaw<sys::obs_properties_t> for ObsProperties {
    unsafe fn from_raw_unchecked(properties: *mut sys::obs_properties_t, guard: Guard) -> Self {
        Self {
            properties,
            guard: guard.unwrap(),
        }
    }
}

impl Drop for ObsProperties {
    fn drop(&mut self) {
        unsafe {
            sys::obs_properties_destroy(self.properties);
        }
    }
}

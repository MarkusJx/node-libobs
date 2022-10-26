use crate::obs::data::obs_property_type::ObsPropertyType;
use crate::obs::sys;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::util::obs_guard::ObsGuard;
use crate::obs::util::types::ResultType;
use core::fmt::Display;
use std::ffi::CStr;
use std::sync::Arc;

pub struct InnerObsProperty {
    property: *mut sys::obs_property_t,
    guard: Arc<ObsGuard>,
}

impl InnerObsProperty {
    pub fn name(&self) -> Option<String> {
        unsafe {
            let name = self.guard.library().ok()?.obs_property_name(self.property);

            if name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name).to_string_lossy().into_owned())
            }
        }
    }

    pub fn description(&self) -> Option<String> {
        unsafe {
            let description = self
                .guard
                .library()
                .ok()?
                .obs_property_description(self.property);

            if description.is_null() {
                None
            } else {
                Some(CStr::from_ptr(description).to_string_lossy().into_owned())
            }
        }
    }

    pub fn long_description(&self) -> Option<String> {
        unsafe {
            let long_description = self
                .guard
                .library()
                .ok()?
                .obs_property_long_description(self.property);

            if long_description.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(long_description)
                        .to_string_lossy()
                        .into_owned(),
                )
            }
        }
    }

    pub fn property_type(&self) -> Option<String> {
        let t = self.get_type().ok()?;
        Some(t.as_ref().to_string())
    }

    pub fn additional_info(&self) -> Option<Vec<String>> {
        let t = self.get_type().ok()?;

        if let ObsPropertyType::List(items) = t {
            Some(items)
        } else {
            None
        }
    }

    pub fn get_type(&self) -> ResultType<ObsPropertyType> {
        ObsPropertyType::try_from(self)
    }

    pub fn get_property_type(&self) -> ResultType<sys::obs_property_type> {
        unsafe { Ok(self.guard.library()?.obs_property_get_type(self.property)) }
    }

    pub fn get_list_items(&self) -> ResultType<Vec<String>> {
        let mut items = Vec::new();

        let library = self.guard.library()?;
        let num = unsafe { library.obs_property_list_item_count(self.property) };
        for i in 0..num {
            let item = unsafe { library.obs_property_list_item_name(self.property, i) };
            if item.is_null() {
                return Err(format!("Failed to get list item with index {}", i).into());
            }

            let item = unsafe { CStr::from_ptr(item) };
            items.push(item.to_string_lossy().into_owned());
        }

        Ok(items)
    }
}

impl FromRaw<sys::obs_property_t> for InnerObsProperty {
    unsafe fn from_raw_unchecked(raw: *mut sys::obs_property_t, guard: Arc<ObsGuard>) -> Self {
        Self {
            property: raw,
            guard,
        }
    }
}

impl Display for InnerObsProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{name: \"{}\", description: \"{}\", longDescription: \"{}\", type: \"{:?}\"}}",
            self.name().unwrap(),
            self.description().unwrap(),
            self.long_description().unwrap_or("".to_string()),
            self.get_type().map_err(|_| std::fmt::Error)?,
        )
    }
}

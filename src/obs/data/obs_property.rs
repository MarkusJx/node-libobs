use crate::obs::data::inner_obs_property::InnerObsProperty;
use crate::obs::sys;
use crate::obs::traits::from_raw::{FromRaw, Guard};

/// An obs property
#[napi(object)]
pub struct ObsProperty {
    /// The name of the property.
    pub name: Option<String>,
    /// The description of the property.
    pub description: Option<String>,
    /// The long description of the property.
    pub long_description: Option<String>,
    /// The type of the property.
    pub property_type: Option<String>,
    /// Additional data of the property.
    /// This may contain the values for a list property.
    pub additional_info: Option<Vec<String>>,
}

impl FromRaw<sys::obs_property_t> for ObsProperty {
    unsafe fn from_raw_unchecked(property: *mut sys::obs_property_t, guard: Guard) -> Self {
        let inner = InnerObsProperty::from_raw(property, guard);
        Self {
            name: inner.name(),
            description: inner.description(),
            long_description: inner.long_description(),
            property_type: inner.property_type(),
            additional_info: inner.additional_info(),
        }
    }
}

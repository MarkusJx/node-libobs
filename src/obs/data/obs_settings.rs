use crate::obs::data::obs_data::ObsData;
use crate::obs::obs::Obs;
use crate::obs::sys;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::traits::raw::Raw;
use crate::obs::util::napi_error::{to_napi_error_str, to_napi_error_string};
use crate::obs::util::node_util::is_integer;
use crate::obs::util::obs_guard::ObsGuard;
use core::fmt::Display;
use napi::{Env, JsObject, JsString, JsUnknown, ValueType};
use std::ffi::{CStr, CString};
use std::sync::Arc;

/// A wrapper around obs settings.
///
/// # Examples
/// ```ts
/// const settings = new ObsSettings({
///     rate_control: 'CQP',
///     cqp: 23,
///     preset: 'medium',
///     profile: 'high',
/// });
/// ```
/// or
/// ```ts
/// const settings = new ObsSettings()
///   .setString('rate_control', 'CQP')
///   .setInt('cqp', 23)
///   .setString('preset', 'medium')
///   .setString('profile', 'high');
/// ```
#[napi]
pub struct ObsSettings(ObsData);

#[napi]
impl ObsSettings {
    /// Create a new settings object.
    /// You can either define the settings in the constructor or use the `set_*` methods.
    ///
    /// @param data - The settings to use.
    #[napi(constructor)]
    pub fn new(
        env: Env,
        #[napi(ts_arg_type = "Record<string, string | number | boolean> | null")] data: Option<
            JsObject,
        >,
        obs: &Obs,
    ) -> napi::Result<Self> {
        let settings = unsafe { obs.library()?.obs_data_create() };

        if settings.is_null() {
            Err(to_napi_error_str("Failed to create settings"))
        } else {
            let mut res = Self(ObsData::from_raw(settings, obs.guard()));
            if let Some(data) = data {
                let keys = data.get_property_names()?;
                for i in 0..keys.get_array_length()? {
                    let key: JsString = keys.get_element(i)?;
                    let key = key.into_utf16()?.as_str()?;

                    let value: JsUnknown = data.get_named_property(&key)?;
                    match value.get_type()? {
                        ValueType::String => {
                            let value: JsString = value.coerce_to_string()?;
                            let value = value.into_utf16()?.as_str()?;
                            res.set_string(key, value)?;
                        }
                        ValueType::Number => {
                            let value = value.coerce_to_number()?;

                            if is_integer(&env, &value)? {
                                res.set_int(key, value.try_into()?)?;
                            } else {
                                res.set_double(key, value.try_into()?)?;
                            }
                        }
                        ValueType::Boolean => {
                            let value = value.coerce_to_bool()?;
                            res.set_bool(key, value.get_value()?)?;
                        }
                        _ => {
                            return Err(to_napi_error_str("Unsupported value type"));
                        }
                    }
                }
            }

            Ok(res)
        }
    }

    pub fn new_empty(obs: &Obs) -> napi::Result<Self> {
        let settings = unsafe { obs.library()?.obs_data_create() };
        Ok(Self(ObsData::from_raw(settings, obs.guard())))
    }

    /// Set a string value.
    #[napi]
    pub fn set_string(&mut self, name: String, value: String) -> napi::Result<&Self> {
        let name = CString::new(name)?;
        let value = CString::new(value)?;

        unsafe {
            self.0
                .library()?
                .obs_data_set_string(self.0.raw(), name.as_ptr(), value.as_ptr());
        }

        Ok(self)
    }

    /// Get a string value.
    #[napi]
    pub fn get_string(&self, name: String) -> napi::Result<String> {
        let name = CString::new(name)?;

        let value = unsafe {
            self.0
                .library()?
                .obs_data_get_string(self.0.raw(), name.as_ptr())
        };

        if value.is_null() {
            Err(to_napi_error_str("Failed to get string"))
        } else {
            let value = unsafe { CStr::from_ptr(value) };
            let value = value
                .to_str()
                .map_err(|e| to_napi_error_string(e.to_string()))?;
            Ok(value.to_string())
        }
    }

    /// Set an integer value.
    #[napi]
    pub fn set_int(&mut self, name: String, value: i64) -> napi::Result<&Self> {
        let name = CString::new(name)?;

        unsafe {
            self.0
                .library()?
                .obs_data_set_int(self.0.raw(), name.as_ptr(), value);
        }

        Ok(self)
    }

    /// Get an integer value.
    #[napi]
    pub fn get_int(&self, name: String) -> napi::Result<i64> {
        let name = CString::new(name)?;

        let value = unsafe {
            self.0
                .library()?
                .obs_data_get_int(self.0.raw(), name.as_ptr())
        };

        Ok(value)
    }

    /// Set a double value.
    #[napi]
    pub fn set_double(&mut self, name: String, value: f64) -> napi::Result<&Self> {
        let name = CString::new(name)?;

        unsafe {
            self.0
                .library()?
                .obs_data_set_double(self.0.raw(), name.as_ptr(), value);
        }

        Ok(self)
    }

    /// Get a double value.
    #[napi]
    pub fn get_double(&self, name: String) -> napi::Result<f64> {
        let name = CString::new(name)?;

        let value = unsafe {
            self.0
                .library()?
                .obs_data_get_double(self.0.raw(), name.as_ptr())
        };

        Ok(value)
    }

    /// Set a boolean value.
    #[napi]
    pub fn set_bool(&mut self, name: String, value: bool) -> napi::Result<&Self> {
        let name = CString::new(name)?;

        unsafe {
            self.0
                .library()?
                .obs_data_set_bool(self.0.raw(), name.as_ptr(), value);
        }

        Ok(self)
    }

    /// Get a boolean value.
    #[napi]
    pub fn get_bool(&self, name: String) -> napi::Result<bool> {
        let name = CString::new(name)?;

        let value = unsafe {
            self.0
                .library()?
                .obs_data_get_bool(self.0.raw(), name.as_ptr())
        };

        Ok(value)
    }
}

impl FromRaw<sys::obs_data_t> for ObsSettings {
    unsafe fn from_raw_unchecked(raw: *mut sys::obs_data_t, guard: Arc<ObsGuard>) -> ObsSettings {
        Self(ObsData::from_raw_unchecked(raw, guard))
    }
}

impl Display for ObsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_json_string().map_err(|_| std::fmt::Error)?
        )
    }
}

impl Raw<sys::obs_data_t> for ObsSettings {
    unsafe fn raw(&self) -> *mut sys::obs_data_t {
        self.0.raw()
    }
}

unsafe impl Send for ObsSettings {}
unsafe impl Sync for ObsSettings {}

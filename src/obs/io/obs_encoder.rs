use crate::obs::data::obs_properties::ObsProperties;
use crate::obs::data::obs_settings::ObsSettings;
use crate::obs::sys;
use crate::obs::traits::from_raw::{FromRaw, Guard};
use crate::obs::traits::raw::Raw;
use crate::obs::util::napi_error::MapToNapiError;
use crate::obs::util::obs_guard::ObsGuard;
use crate::obs::util::types::ResultType;
use std::sync::Arc;

struct ObsEncoder {
    encoder: *mut sys::obs_encoder_t,
    guard: Arc<ObsGuard>,
}

impl ObsEncoder {
    pub fn get_settings(&self) -> ResultType<ObsSettings> {
        let data = unsafe { sys::obs_encoder_get_settings(self.encoder) };

        if data.is_null() {
            Err("Failed to get encoder settings".into())
        } else {
            Ok(ObsSettings::from_raw(data, Some(self.guard.clone())))
        }
    }

    pub fn update(&self, settings: &ObsSettings) {
        unsafe {
            sys::obs_encoder_update(self.encoder, settings.raw());
        }
    }

    pub fn get_properties(&self) -> ResultType<ObsProperties> {
        let data = unsafe { sys::obs_encoder_properties(self.encoder) };

        if data.is_null() {
            Err("Failed to get encoder properties".into())
        } else {
            Ok(ObsProperties::from_raw(data, Some(self.guard.clone())))
        }
    }
}

/// An obs video encoder.
///
/// # Example
/// ```ts
/// const encoder = await obs.createVideoEncoder('nvenc', 'jim_nvenc', new ObsSettings({
///    rate_control: 'CQP',
///    cqp: 23,
///    preset: 'medium',
///    profile: 'high',
/// }));
#[napi]
pub struct ObsVideoEncoder(ObsEncoder);

#[napi]
impl ObsVideoEncoder {
    /// Get the settings of this encoder.
    #[napi]
    pub fn get_settings(&self) -> napi::Result<ObsSettings> {
        self.0.get_settings().map_napi_err()
    }

    /// Update the settings of this encoder.
    #[napi]
    pub fn update_settings(&self, settings: &ObsSettings) {
        self.0.update(settings)
    }

    /// Get the properties of this encoder.
    #[napi]
    pub fn get_properties(&self) -> napi::Result<ObsProperties> {
        self.0.get_properties().map_napi_err()
    }
}

impl FromRaw<sys::obs_encoder_t> for ObsVideoEncoder {
    unsafe fn from_raw_unchecked(encoder: *mut sys::obs_encoder_t, guard: Guard) -> Self {
        Self(ObsEncoder {
            encoder,
            guard: guard.unwrap(),
        })
    }
}

unsafe impl Send for ObsVideoEncoder {}

/// An obs audio encoder.
///
/// # Example
/// ```ts
/// const audioEncoder = await obs.createAudioEncoder('aac', 'ffmpeg_aac', new ObsSettings({
///    bitrate: 128,
///    rate_control: 'CBR',
/// }));
/// ```
#[napi]
pub struct ObsAudioEncoder(ObsEncoder);

#[napi]
impl ObsAudioEncoder {
    /// Get the settings of this encoder.
    #[napi]
    pub fn get_settings(&self) -> napi::Result<ObsSettings> {
        self.0.get_settings().map_napi_err()
    }

    /// Update the settings of this encoder.
    #[napi]
    pub fn update_settings(&self, settings: &ObsSettings) {
        self.0.update(settings)
    }

    /// Get the properties of this encoder.
    #[napi]
    pub fn get_properties(&self) -> napi::Result<ObsProperties> {
        self.0.get_properties().map_napi_err()
    }
}

impl FromRaw<sys::obs_encoder_t> for ObsAudioEncoder {
    unsafe fn from_raw_unchecked(encoder: *mut sys::obs_encoder_t, guard: Guard) -> Self {
        Self(ObsEncoder {
            encoder,
            guard: guard.unwrap(),
        })
    }
}

unsafe impl Send for ObsAudioEncoder {}

impl Raw<sys::obs_encoder_t> for ObsVideoEncoder {
    unsafe fn raw(&self) -> *mut sys::obs_encoder_t {
        self.0.encoder
    }
}

impl Raw<sys::obs_encoder_t> for ObsAudioEncoder {
    unsafe fn raw(&self) -> *mut sys::obs_encoder_t {
        self.0.encoder
    }
}

impl Drop for ObsEncoder {
    fn drop(&mut self) {
        unsafe {
            sys::obs_encoder_release(self.encoder);
        }
    }
}

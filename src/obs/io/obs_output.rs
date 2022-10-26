use crate::obs::data::obs_properties::ObsProperties;
use crate::obs::data::obs_settings::ObsSettings;
use crate::obs::io::obs_encoder::{ObsAudioEncoder, ObsVideoEncoder};
use crate::obs::sys;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::traits::raw::Raw;
use crate::obs::util::napi_error::{to_napi_error_str, to_napi_error_string};
use crate::obs::util::obs_guard::ObsGuard;
use std::ffi::CStr;
use std::mem;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
enum OutputState {
    Stopped,
    Running,
    Paused,
}

/// An obs output.
#[napi]
pub struct ObsOutput {
    output: AtomicPtr<sys::obs_output_t>,
    state: Mutex<OutputState>,
    guard: Arc<ObsGuard>,
}

#[napi]
impl ObsOutput {
    /// Start the output.
    /// This will start the output and start sending data to the output.
    /// At least one encoder must be set before starting the output.
    /// Throws an error if the output is already running.
    #[napi]
    pub fn start(
        &self,
        video_encoder: Option<&ObsVideoEncoder>,
        audio_encoder: Option<&ObsAudioEncoder>,
    ) -> napi::Result<()> {
        let mut state = self.state.lock().unwrap();
        if *state == OutputState::Running {
            return Err(to_napi_error_str("Output is already running"));
        }

        if video_encoder.is_none() && audio_encoder.is_none() {
            return Err(to_napi_error_str("No encoder specified"));
        }

        let library = self.guard.library()?;
        if let Some(encoder) = video_encoder {
            let video = unsafe { library.obs_get_video() };
            if video.is_null() {
                return Err(to_napi_error_str("Failed to get video"));
            }

            unsafe {
                library.obs_encoder_set_video(encoder.raw(), video);
                library.obs_output_set_video_encoder(self.raw(), encoder.raw());
            }
        }

        if let Some(encoder) = audio_encoder {
            let audio = unsafe { library.obs_get_audio() };
            if audio.is_null() {
                return Err(to_napi_error_str("Failed to get audio"));
            }

            unsafe {
                library.obs_encoder_set_audio(encoder.raw(), audio);
                library.obs_output_set_audio_encoder(self.raw(), encoder.raw(), 0);
            }
        }

        unsafe {
            library.obs_output_set_media(
                self.raw(),
                library.obs_get_video(),
                library.obs_get_audio(),
            );
        }

        let ok = unsafe { library.obs_output_start(self.raw()) };

        if !ok {
            let error = unsafe { library.obs_output_get_last_error(self.raw()) };
            let message: String = if error.is_null() {
                "Unknown".to_string()
            } else {
                unsafe { CStr::from_ptr(error) }
                    .to_string_lossy()
                    .into_owned()
            };

            Err(to_napi_error_string(format!(
                "Failed to start output. Error message: {}",
                message
            )))
        } else {
            let _ = mem::replace(&mut *state, OutputState::Running);
            Ok(())
        }
    }

    /// Stop the output.
    /// This will stop the output and stop sending data to the output.
    /// Throws an error if the output is not running.
    #[napi]
    pub fn stop(&self) -> napi::Result<()> {
        let mut state = self.state.lock().unwrap();
        if *state == OutputState::Stopped {
            return Err(to_napi_error_str("Output is already stopped"));
        }

        unsafe { self.guard.library()?.obs_output_stop(self.raw()) };
        let _ = mem::replace(&mut *state, OutputState::Stopped);
        Ok(())
    }

    /// Force stop the output.
    /// This will force stop the output and stop sending data to the output.
    /// Throws an error if the output is not running.
    #[napi]
    pub fn force_stop(&self) -> napi::Result<()> {
        let mut state = self.state.lock().unwrap();
        if *state == OutputState::Stopped {
            return Err(to_napi_error_str("Output is already stopped"));
        }

        unsafe { self.guard.library()?.obs_output_force_stop(self.raw()) };
        let _ = mem::replace(&mut *state, OutputState::Stopped);
        Ok(())
    }

    /// Get the output properties.
    #[napi]
    pub fn get_properties(&self) -> napi::Result<ObsProperties> {
        let properties = unsafe { self.guard.library()?.obs_output_properties(self.raw()) };
        if properties.is_null() {
            return Err(to_napi_error_str("Failed to get properties"));
        }

        Ok(ObsProperties::from_raw(properties, self.guard.clone()))
    }

    /// Get if the output is paused.
    #[napi(getter)]
    pub fn get_paused(&self) -> napi::Result<bool> {
        let _lock = self.state.lock().unwrap();
        unsafe { Ok(self.guard.library()?.obs_output_paused(self.raw())) }
    }

    /// Check if the output can be paused.
    #[napi(getter)]
    pub fn can_pause(&self) -> napi::Result<bool> {
        unsafe { Ok(self.guard.library()?.obs_output_can_pause(self.raw())) }
    }

    /// Check if the output is active.
    #[napi(getter)]
    pub fn get_active(&self) -> napi::Result<bool> {
        unsafe { Ok(self.guard.library()?.obs_output_active(self.raw())) }
    }

    /// Get the output name.
    #[napi(getter)]
    pub fn get_name(&self) -> napi::Result<String> {
        Ok(
            unsafe { CStr::from_ptr(self.guard.library()?.obs_output_get_name(self.raw())) }
                .to_string_lossy()
                .into_owned(),
        )
    }

    /// Pause the output.
    /// Throws an error if the output is not running.
    #[napi]
    pub fn pause(&self) -> napi::Result<()> {
        let mut state = self.state.lock().unwrap();
        if *state == OutputState::Stopped || *state == OutputState::Paused {
            return Err(to_napi_error_str("The output is not running"));
        }

        println!("Pausing output: {:?}", unsafe { *self.raw() });
        if !unsafe { self.guard.library()?.obs_output_pause(self.raw(), true) } {
            Err(to_napi_error_str("Failed to pause output"))
        } else {
            let _ = mem::replace(&mut *state, OutputState::Paused);
            Ok(())
        }
    }

    /// Resume the output.
    /// Throws an error if the output is not paused.
    #[napi]
    pub fn resume(&self) -> napi::Result<()> {
        let mut state = self.state.lock().unwrap();
        if *state != OutputState::Paused {
            return Err(to_napi_error_str("The output is not paused"));
        }

        if !unsafe { self.guard.library()?.obs_output_pause(self.raw(), false) } {
            Err(to_napi_error_str("Failed to resume output"))
        } else {
            let _ = mem::replace(&mut *state, OutputState::Running);
            Ok(())
        }
    }

    /// Set the output settings.
    #[napi(setter)]
    pub fn set_settings(&self, settings: &ObsSettings) -> napi::Result<()> {
        unsafe {
            self.guard
                .library()?
                .obs_output_update(self.raw(), settings.raw())
        };

        Ok(())
    }

    /// Get the output settings.
    #[napi(getter)]
    pub fn get_settings(&self) -> napi::Result<ObsSettings> {
        let settings = unsafe { self.guard.library()?.obs_output_get_settings(self.raw()) };
        if settings.is_null() {
            return Err(to_napi_error_str("Failed to get settings"));
        } else {
            Ok(ObsSettings::from_raw(settings, self.guard.clone()))
        }
    }
}

impl FromRaw<sys::obs_output_t> for ObsOutput {
    unsafe fn from_raw_unchecked(raw: *mut sys::obs_output_t, guard: Arc<ObsGuard>) -> Self {
        Self {
            output: AtomicPtr::new(raw),
            state: Mutex::new(OutputState::Stopped),
            guard,
        }
    }
}

impl Raw<sys::obs_output_t> for ObsOutput {
    unsafe fn raw(&self) -> *mut sys::obs_output_t {
        self.output.load(Ordering::Relaxed)
    }
}

unsafe impl Send for ObsOutput {}

impl Drop for ObsOutput {
    fn drop(&mut self) {
        if let Ok(library) = self.guard.library() {
            unsafe {
                //library.obs_output_release(self.raw());
            }
        }
    }
}

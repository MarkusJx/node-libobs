use crate::obs::data::loaded_obs_module::LoadedObsModule;
use crate::obs::data::obs_module::ObsModule;
use crate::obs::data::obs_settings::ObsSettings;
use crate::obs::io::obs_encoder::{ObsAudioEncoder, ObsVideoEncoder};
use crate::obs::io::obs_output::ObsOutput;
use crate::obs::io::obs_source::ObsSource;
use crate::obs::objects::failed_obs_module::FailedObsModule;
use crate::obs::objects::reset_audio_data::ResetAudioData;
use crate::obs::objects::reset_video_data::ResetVideoData;
use crate::obs::sys;
use crate::obs::traits::enum_value::EnumValue;
use crate::obs::traits::from_raw::FromRaw;
use crate::obs::traits::raw::Raw;
use crate::obs::util::napi_error::{to_napi_error_str, to_napi_error_string, MapToNapiError};
use crate::obs::util::obs_error::{obs_error_to_string, OBS_VIDEO_SUCCESS};
use crate::obs::util::obs_guard::ObsGuard;
use futures::future;
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Arc;

/// The main obs class.
/// You can only have one instance of this class active at a time.
///
/// # Example
/// ```ts
/// const obs = await Obs.newInstance('en-US');
///
/// // Get all modules which may be loaded
/// const modules = await obs.getAllModules('/path/to/your/obs/installation');
/// // Load the modules
/// await obs.loadModules(modules);
///
/// // Reset the audio and video
/// await obs.resetAudio({
///    fixedBuffering: false,
///    speakers: obs.SpeakerLayout.Stereo,
///    maxBufferingMs: 1000,
///    samplesPerSec: 48000,
/// });
///
/// await obs.resetVideo({
///    adapter: 0,
///    baseHeight: 1440,
///    baseWidth: 2560,
///    outputHeight: 1440,
///    outputWidth: 2560,
///    scaleType: obs.ScaleType.Bicubic,
///    colorspace: obs.VideoColorSpace.CS709,
///    fpsDen: 1,
///    fpsNum: 60,
///    gpuConversion: true,
///    range: obs.VideoRange.Partial,
///    graphicsModule: obs.GraphicsModule.D3D11,
///    outputFormat: obs.VideoFormat.NV12,
/// });
///
/// // Create a video encoder
/// const videoEncoder = await obs.createVideoEncoder('nvenc', 'jim_nvenc', new ObsSettings({
///     rate_control: 'CQP',
///     cqp: 23,
///     preset: 'medium',
///     profile: 'high',
/// ));
///
/// // Create an audio encoder
/// const audioEncoder = await obs.createAudioEncoder('aac', 'jim_aac', new ObsSettings({
///    bitrate: 128,
///    rate_control: 'CBR',
/// }));
///
/// // Create a new video source
/// const videoSource = await obs.createSource('screen_capture', 'monitor_capture', 0, new ObsSettings({
///    capture_cursor: true,
///    monitor: 0,
///    method: 2,
/// }));
///
/// // Create a new audio source
/// await obs.createSource('audio_capture', 'wasapi_output_capture', 1);
///
/// // Create a new flv output
/// const out = await obs.createOutput('flv_output', 'output', new ObsSettings({
///    path: '/path/to/your/output.flv',
/// }));
///
/// // Start the output
/// out.start(videoEncoder, audioEncoder);
///
/// // Wait for 10 seconds
/// await new Promise(resolve => setTimeout(resolve, 10000));
///
/// // Stop the output
/// out.stop();
/// ```
#[napi]
pub struct Obs {
    guard: Arc<ObsGuard>,
    failed_modules: Vec<FailedObsModule>,
}

#[napi]
impl Obs {
    /// Create a new OBS instance.
    #[napi(constructor)]
    pub fn new(locale: String) -> napi::Result<Self> {
        let locale = CString::new(locale)?;

        let initialized: bool =
            unsafe { sys::obs_startup(locale.as_ptr(), ptr::null_mut(), ptr::null_mut()) };

        if initialized {
            Ok(Self {
                guard: Arc::new(ObsGuard::new()),
                failed_modules: Vec::new(),
            })
        } else {
            Err(to_napi_error_str("Failed to initialize OBS"))
        }
    }

    /// Create a new OBS instance.
    /// Async version.
    #[napi(js_name = "newInstance")]
    pub async fn new_obs_instance(locale: String) -> napi::Result<Obs> {
        future::lazy(move |_| Self::new(locale)).await
    }

    /// Get all modules which may be loaded.
    /// This may include 'modules' which are not in fact modules.
    #[napi]
    pub fn get_all_modules_sync(&self, obs_path: String) -> napi::Result<Vec<ObsModule>> {
        ObsModule::get_all_modules(obs_path)
    }

    /// Get all modules which may be loaded.
    /// Async version of `getAllModulesSync`.
    #[napi]
    pub async fn get_all_modules(&self, obs_path: String) -> napi::Result<Vec<ObsModule>> {
        future::lazy(move |_| ObsModule::get_all_modules(obs_path)).await
    }

    /// Load modules.
    ///
    /// @param modules - the modules to load
    /// @param throwOnLoadFailed - whether to throw an error if a module fails to load. Defaults to `false`.
    #[napi]
    pub fn load_modules_sync(
        &mut self,
        modules: Vec<ObsModule>,
        throw_on_load_failed: Option<bool>,
    ) -> napi::Result<()> {
        for module in modules {
            let res = module.load();

            if throw_on_load_failed.unwrap_or(false) {
                res.map_napi_err()?;
            } else if res.is_err() {
                self.failed_modules
                    .push(FailedObsModule::new(module, res.err().unwrap().to_string()));
            }
        }

        Ok(())
    }

    /// Load modules.
    /// Async version of `loadModulesSync`.
    #[napi]
    pub async fn load_modules(
        &mut self,
        modules: Vec<ObsModule>,
        throw_on_load_failed: Option<bool>,
    ) -> napi::Result<()> {
        future::lazy(move |_| self.load_modules_sync(modules, throw_on_load_failed)).await
    }

    #[napi]
    pub async fn init_audio_monitoring(
        &self,
        device_name: String,
        device_id: String,
    ) -> napi::Result<bool> {
        if unsafe { sys::obs_audio_monitoring_available() } {
            let device_name = CString::new(device_name)?;
            let device_id = CString::new(device_id)?;

            let ok = unsafe {
                sys::obs_set_audio_monitoring_device(device_name.as_ptr(), device_id.as_ptr())
            };

            if ok {
                Ok(true)
            } else {
                Err(to_napi_error_str("Failed to initialize audio monitoring"))
            }
        } else {
            Ok(false)
        }
    }

    /// Reset the video data.
    #[napi]
    pub fn reset_video_sync(&self, data: ResetVideoData) -> napi::Result<()> {
        let graphics_module = CString::new(data.graphics_module.to_string())?;
        let mut info = sys::obs_video_info {
            graphics_module: graphics_module.as_ptr(),
            fps_num: data.fps_num,
            fps_den: data.fps_den,
            base_width: data.base_width,
            base_height: data.base_height,
            output_width: data.output_width,
            output_height: data.output_height,
            output_format: data.output_format.value(),
            adapter: data.adapter,
            gpu_conversion: data.gpu_conversion,
            colorspace: data.colorspace.value(),
            range: data.range.value(),
            scale_type: data.scale_type.value(),
        };

        let res = unsafe { sys::obs_reset_video(&mut info as *mut _) };

        if res == OBS_VIDEO_SUCCESS {
            Ok(())
        } else {
            Err(to_napi_error_string(format!(
                "Failed to set video. Error: {}",
                obs_error_to_string(res)
            )))
        }
    }

    /// Reset the video data.
    /// Async version of `resetVideoSync`.
    #[napi]
    pub async fn reset_video(&'static self, data: ResetVideoData) -> napi::Result<()> {
        future::lazy(move |_| self.reset_video_sync(data)).await
    }

    /// Reset the audio data.
    #[napi]
    pub fn reset_audio_sync(&self, data: ResetAudioData) -> napi::Result<()> {
        let res = unsafe {
            let mut info = sys::obs_audio_info2 {
                samples_per_sec: data.samples_per_sec,
                speakers: data.speakers.value(),
                max_buffering_ms: data.max_buffering_ms,
                fixed_buffering: data.fixed_buffering,
            };

            sys::obs_reset_audio2(&mut info as *mut _)
        };

        if res {
            Ok(())
        } else {
            Err(to_napi_error_str("Failed to reset audio"))
        }
    }

    /// Reset the audio data.
    /// Async version of `resetAudioSync`.
    #[napi]
    pub async fn reset_audio(&'static self, data: ResetAudioData) -> napi::Result<()> {
        future::lazy(move |_| self.reset_audio_sync(data)).await
    }

    /// Get a list of modules which failed to load.
    #[napi(getter)]
    pub fn failed_modules(&self) -> napi::Result<Vec<FailedObsModule>> {
        Ok(self.failed_modules.clone())
    }

    #[napi(getter)]
    pub fn get_loaded_modules(&self) -> napi::Result<Vec<LoadedObsModule>> {
        LoadedObsModule::list_loaded_modules()
    }

    /// List all encoder types.
    /// This list includes video and audio encoders.
    #[napi]
    pub fn list_encoder_types_sync(&self) -> napi::Result<Vec<String>> {
        let mut ok = true;
        let mut i: u64 = 0;
        let mut res = vec![];

        unsafe {
            while ok {
                let mut ptr: *mut std::os::raw::c_char = ptr::null_mut();
                ok = sys::obs_enum_encoder_types(
                    i,
                    &mut ptr as *mut *mut std::os::raw::c_char as *mut *const std::os::raw::c_char,
                );
                i += 1;

                if ok && !ptr.is_null() {
                    let cstr = CStr::from_ptr(ptr);
                    res.push(cstr.to_string_lossy().to_string());
                }
            }
        }

        Ok(res)
    }

    /// List all encoder types.
    /// Async version of `listEncoderTypesSync`.
    #[napi]
    pub async fn list_encoder_types(&self) -> napi::Result<Vec<String>> {
        future::lazy(move |_| self.list_encoder_types_sync()).await
    }

    /// List all output types.
    #[napi]
    pub fn list_output_types_sync(&self) -> napi::Result<Vec<String>> {
        let mut ok = true;
        let mut i: u64 = 0;
        let mut res = vec![];

        unsafe {
            while ok {
                let mut ptr: *mut std::os::raw::c_char = ptr::null_mut();
                ok = sys::obs_enum_output_types(
                    i,
                    &mut ptr as *mut *mut std::os::raw::c_char as *mut *const std::os::raw::c_char,
                );
                i += 1;

                if ok && !ptr.is_null() {
                    let cstr = CStr::from_ptr(ptr);
                    res.push(cstr.to_string_lossy().to_string());
                }
            }
        }

        Ok(res)
    }

    /// List all output types.
    /// Async version of `listOutputTypesSync`.
    #[napi]
    pub async fn list_output_types(&self) -> napi::Result<Vec<String>> {
        future::lazy(|_| self.list_output_types_sync()).await
    }

    /*#[napi]
    pub fn list_service_types_sync(&self) -> napi::Result<Vec<String>> {
        let mut ok = true;
        let mut i: u64 = 0;
        let mut res = vec![];

        unsafe {
            while ok {
                let mut ptr: *mut std::os::raw::c_char = ptr::null_mut();
                ok = sys::obs_enum_service_types(
                    i,
                    &mut ptr as *mut *mut std::os::raw::c_char as *mut *const std::os::raw::c_char,
                );
                i += 1;

                if ok && !ptr.is_null() {
                    let cstr = CStr::from_ptr(ptr);
                    res.push(cstr.to_string_lossy().to_string());
                }
            }
        }

        Ok(res)
    }

    #[napi]
    pub async fn list_service_types(&self) -> napi::Result<Vec<String>> {
        future::lazy(move |_| self.list_service_types_sync()).await
    }*/

    #[napi]
    pub fn list_source_types_sync(&self) -> napi::Result<Vec<String>> {
        let mut ok = true;
        let mut i: u64 = 0;
        let mut res = vec![];

        unsafe {
            while ok {
                let mut ptr: *mut std::os::raw::c_char = ptr::null_mut();
                ok = sys::obs_enum_source_types(
                    i,
                    &mut ptr as *mut *mut std::os::raw::c_char as *mut *const std::os::raw::c_char,
                );
                i += 1;

                if ok && !ptr.is_null() {
                    let cstr = CStr::from_ptr(ptr);
                    res.push(cstr.to_string_lossy().to_string());
                }
            }
        }

        Ok(res)
    }

    #[napi]
    pub async fn list_source_types(&self) -> napi::Result<Vec<String>> {
        future::lazy(|_| self.list_source_types_sync()).await
    }

    #[napi]
    pub fn create_video_encoder_sync(
        &self,
        name: String,
        id: String,
        settings: Option<&ObsSettings>,
    ) -> napi::Result<ObsVideoEncoder> {
        let id = CString::new(id)?;
        let name = CString::new(name)?;

        let encoder = unsafe {
            sys::obs_video_encoder_create(
                id.as_ptr(),
                name.as_ptr(),
                settings.map(|s| s.raw()).unwrap_or(ptr::null_mut()),
                ptr::null_mut(),
            )
        };

        if encoder.is_null() {
            Err(to_napi_error_str("Failed to get encoder"))
        } else {
            Ok(ObsVideoEncoder::from_raw(encoder, Some(self.guard.clone())))
        }
    }

    #[napi]
    pub async fn create_video_encoder(
        &'static self,
        name: String,
        id: String,
        settings: Option<&'static ObsSettings>,
    ) -> napi::Result<ObsVideoEncoder> {
        future::lazy(move |_| self.create_video_encoder_sync(id, name, settings)).await
    }

    #[napi]
    pub fn create_audio_encoder_sync(
        &self,
        name: String,
        id: String,
        settings: Option<&ObsSettings>,
    ) -> napi::Result<ObsAudioEncoder> {
        let id = CString::new(id)?;
        let name = CString::new(name)?;
        let encoder = unsafe {
            sys::obs_audio_encoder_create(
                id.as_ptr(),
                name.as_ptr(),
                settings.map(|s| s.raw()).unwrap_or(ptr::null_mut()),
                0,
                ptr::null_mut(),
            )
        };

        if encoder.is_null() {
            Err(to_napi_error_str("Failed to get encoder"))
        } else {
            Ok(ObsAudioEncoder::from_raw(encoder, Some(self.guard.clone())))
        }
    }

    #[napi]
    pub async fn create_audio_encoder(
        &'static self,
        name: String,
        id: String,
        settings: Option<&'static ObsSettings>,
    ) -> napi::Result<ObsAudioEncoder> {
        future::lazy(move |_| self.create_audio_encoder_sync(id, name, settings)).await
    }

    /*pub fn get_audio_monitoring_device(&self) -> napi::Result<AudioDevice> {
        let mut name: *const std::os::raw::c_char = ptr::null();
        let mut id: *const std::os::raw::c_char = ptr::null();
        unsafe {
            sys::obs_get_audio_monitoring_device(&mut name as _, &mut id as _);
        }

        if name.is_null() || id.is_null() {
            Err("Failed to get audio monitoring device".into())
        } else {
            let name = unsafe { CStr::from_ptr(name) };
            let id = unsafe { CStr::from_ptr(id) };

            Ok(AudioDevice {
                name: name.to_string_lossy().to_string(),
                id: id.to_string_lossy().to_string(),
            })
        }
    }*/

    #[napi]
    pub fn create_output_sync(
        &self,
        name: String,
        id: String,
        settings: Option<&ObsSettings>,
    ) -> napi::Result<ObsOutput> {
        let id = CString::new(id)?;
        let name = CString::new(name)?;
        let output = unsafe {
            sys::obs_output_create(
                id.as_ptr(),
                name.as_ptr(),
                settings.map(|s| s.raw()).unwrap_or(ptr::null_mut()),
                ptr::null_mut(),
            )
        };

        if output.is_null() {
            Err(to_napi_error_str("Failed to get encoder"))
        } else {
            Ok(ObsOutput::from_raw(output, Some(self.guard.clone())))
        }
    }

    #[napi]
    pub async fn create_output(
        &'static self,
        name: String,
        id: String,
        settings: Option<&'static ObsSettings>,
    ) -> napi::Result<ObsOutput> {
        future::lazy(|_| self.create_output_sync(id, name, settings)).await
    }

    #[napi]
    pub fn create_source_sync(
        &self,
        name: String,
        id: String,
        channel: u32,
        settings: Option<&ObsSettings>,
    ) -> napi::Result<ObsSource> {
        let source = unsafe {
            let name = CString::new(name)?;
            let id = CString::new(id)?;

            sys::obs_source_create(
                id.as_ptr(),
                name.as_ptr(),
                settings.map(|s| s.raw()).unwrap_or(ptr::null_mut()),
                ptr::null_mut(),
            )
        };

        unsafe {
            sys::obs_set_output_source(channel, source);
        }

        if source.is_null() {
            Err(to_napi_error_str("Failed to create source"))
        } else {
            Ok(ObsSource::from_raw(source, Some(self.guard.clone())))
        }
    }

    #[napi]
    pub async fn create_source(
        &'static self,
        name: String,
        id: String,
        channel: u32,
        settings: Option<&'static ObsSettings>,
    ) -> napi::Result<ObsSource> {
        future::lazy(|_| self.create_source_sync(name, id, channel, settings)).await
    }
}

unsafe impl Send for Obs {}

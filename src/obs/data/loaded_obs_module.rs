use crate::obs::sys;
use crate::obs::util::napi_error::to_napi_error_str;
use crate::obs::util::obs_guard::ObsGuard;
use std::ffi::CStr;
use std::sync::{Arc, MutexGuard};

extern "C" fn enum_module(param: *mut std::os::raw::c_void, data: *mut sys::obs_module_t) {
    let module_data = unsafe { &mut *(param as *mut ModuleData) };
    let module = LoadedObsModule::new(data, &module_data.library);

    module_data.modules.push(module);
}

struct ModuleData<'a> {
    modules: Vec<napi::Result<LoadedObsModule>>,
    library: &'a MutexGuard<'a, sys::Bindings>,
}

/// A loaded OBS module.
#[napi(object)]
pub struct LoadedObsModule {
    pub name: Option<String>,
    pub file_name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub binary_path: String,
    pub data_path: String,
}

impl LoadedObsModule {
    fn new(
        module: *mut sys::obs_module_t,
        library: &MutexGuard<sys::Bindings>,
    ) -> napi::Result<Self> {
        let name = unsafe { library.obs_get_module_name(module) };
        let file_name = unsafe { library.obs_get_module_file_name(module) };
        let description = unsafe { library.obs_get_module_description(module) };
        let author = unsafe { library.obs_get_module_author(module) };
        let binary_path = unsafe { library.obs_get_module_binary_path(module) };
        let data_path = unsafe { library.obs_get_module_data_path(module) };

        let to_string = |ptr: *const i8| {
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string())
            }
        };

        Ok(Self {
            name: to_string(name),
            file_name: to_string(file_name)
                .ok_or(to_napi_error_str("Could not get the module file name"))?,
            description: to_string(description),
            author: to_string(author),
            binary_path: to_string(binary_path)
                .ok_or(to_napi_error_str("Could not get the module binary path"))?,
            data_path: to_string(data_path)
                .ok_or(to_napi_error_str("Could not get the module data path"))?,
        })
    }

    pub fn list_loaded_modules(guard: &Arc<ObsGuard>) -> napi::Result<Vec<Self>> {
        let library = guard.library()?;
        let mut module_data = ModuleData {
            modules: Vec::new(),
            library: &library,
        };

        unsafe {
            library.obs_enum_modules(Some(enum_module), &mut module_data as *mut _ as *mut _);
        }

        module_data.modules.into_iter().collect::<_>()
    }
}

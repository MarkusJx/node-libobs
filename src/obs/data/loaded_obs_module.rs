use crate::obs::sys;
use crate::obs::util::napi_error::to_napi_error_str;
use std::ffi::CStr;

extern "C" fn enum_module(param: *mut std::os::raw::c_void, data: *mut sys::obs_module_t) {
    let modules = unsafe { &mut *(param as *mut Vec<napi::Result<LoadedObsModule>>) };
    let module = LoadedObsModule::new(data);

    modules.push(module);
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
    fn new(module: *mut sys::obs_module_t) -> napi::Result<Self> {
        let name = unsafe { sys::obs_get_module_name(module) };
        let file_name = unsafe { sys::obs_get_module_file_name(module) };
        let description = unsafe { sys::obs_get_module_description(module) };
        let author = unsafe { sys::obs_get_module_author(module) };
        let binary_path = unsafe { sys::obs_get_module_binary_path(module) };
        let data_path = unsafe { sys::obs_get_module_data_path(module) };

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

    pub fn list_loaded_modules() -> napi::Result<Vec<Self>> {
        let mut modules = Vec::<napi::Result<Self>>::new();
        unsafe {
            sys::obs_enum_modules(Some(enum_module), &mut modules as *mut _ as *mut _);
        }

        modules.into_iter().collect::<_>()
    }
}

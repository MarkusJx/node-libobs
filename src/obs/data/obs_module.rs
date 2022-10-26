use crate::obs::sys;
use crate::obs::util::napi_error::to_napi_error_str;
use crate::obs::util::obs_error::{module_error_to_string, OBS_MODULE_SUCCESS};
use crate::obs::util::types::ResultType;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::sync::MutexGuard;

extern "C" fn get_module(param: *mut std::ffi::c_void, info: *const sys::obs_module_info2) {
    let modules = unsafe { &mut *(param as *mut Vec<ObsModule>) };

    if let Ok(module) = unsafe { ObsModule::new(&*info) } {
        modules.push(module);
    } else {
        let name = if unsafe { (*info).name.is_null() } {
            "null".to_string()
        } else {
            unsafe {
                CStr::from_ptr((*info).name as *mut i8)
                    .to_string_lossy()
                    .to_string()
            }
        };

        println!("Failed to get info for module '{}'", name);
    }
}

/// An obs module.
#[napi(object)]
#[derive(Debug, Clone)]
pub struct ObsModule {
    /// The name of the module.
    pub name: String,
    /// The path of the module binary.
    pub bin_path: String,
    /// The path of the module data.
    pub data_path: String,
}

#[cfg(target_os = "windows")]
mod separators {
    pub const PATH_SEP: &str = "\\";
    pub const SEP_TO_REPLACE: &str = "/";
}
#[cfg(not(target_os = "windows"))]
mod separators {
    pub const PATH_SEP: &str = "/";
    pub const SEP_TO_REPLACE: &str = "\\";
}

impl ObsModule {
    pub fn get_all_modules(
        library: MutexGuard<sys::Bindings>,
        obs_path: String,
    ) -> napi::Result<Vec<ObsModule>> {
        let mut modules = Vec::new();
        unsafe {
            let path = Path::new(obs_path.as_str());
            let bin = CString::new(
                path.join("obs-plugins")
                    .join("64bit")
                    .to_str()
                    .ok_or(to_napi_error_str("Invalid path"))?,
            )?;
            let data = CString::new(
                path.join("data")
                    .join("obs-plugins")
                    .to_str()
                    .ok_or(to_napi_error_str("Invalid path"))?,
            )?;

            library.obs_add_module_path(bin.as_ptr(), data.as_ptr());
            library.obs_find_modules2(Some(get_module), &mut modules as *mut _ as *mut _);
        }

        Ok(modules)
    }

    unsafe fn new(info: &sys::obs_module_info2) -> ResultType<Self> {
        if info.name.is_null() || info.bin_path.is_null() || info.data_path.is_null() {
            return Err("Invalid module info".into());
        }

        let name = CStr::from_ptr(info.name);
        let bin_path = CStr::from_ptr(info.bin_path);
        let data_path = CStr::from_ptr(info.data_path);

        Ok(Self {
            name: name.to_str()?.to_string(),
            bin_path: bin_path
                .to_str()?
                .replace(separators::SEP_TO_REPLACE, separators::PATH_SEP),
            data_path: data_path
                .to_str()?
                .replace(separators::SEP_TO_REPLACE, separators::PATH_SEP),
        })
    }

    pub fn load(&self, library: MutexGuard<sys::Bindings>) -> ResultType<()> {
        let mut module: *mut sys::obs_module_t = ptr::null_mut();
        let path = CString::new(self.bin_path.as_str())?;
        let data = CString::new(self.data_path.as_str())?;

        let open_res =
            unsafe { library.obs_open_module(&mut module, path.as_ptr(), data.as_ptr()) };

        if (open_res != OBS_MODULE_SUCCESS || module.is_null())
            && open_res != sys::MODULE_HARDCODED_SKIP
        {
            return Err(format!(
                "Failed to load module '{}': {}",
                self.name,
                module_error_to_string(open_res)
            )
            .into());
        } else if open_res == sys::MODULE_HARDCODED_SKIP {
            println!("Module {} is hardcoded to be skipped", self.name);
            return Ok(());
        }

        if unsafe { library.obs_init_module(module) } {
            Ok(())
        } else {
            Err(format!("Failed to init module '{}'", self.name).into())
        }
    }
}

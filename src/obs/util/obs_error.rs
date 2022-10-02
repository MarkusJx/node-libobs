use crate::obs::sys;

pub const OBS_VIDEO_SUCCESS: i32 = sys::OBS_VIDEO_SUCCESS as _;
pub const OBS_MODULE_SUCCESS: i32 = sys::MODULE_SUCCESS as _;

pub fn obs_error_to_string(code: i32) -> &'static str {
    match code {
        OBS_VIDEO_SUCCESS => "success",
        sys::OBS_VIDEO_FAIL => "generic failure",
        sys::OBS_VIDEO_MODULE_NOT_FOUND => "video module not found",
        sys::OBS_VIDEO_CURRENTLY_ACTIVE => "video is currently active",
        sys::OBS_VIDEO_INVALID_PARAM => "invalid param",
        sys::OBS_VIDEO_NOT_SUPPORTED => "not supported",
        _ => "unknown error",
    }
}

pub fn module_error_to_string(code: i32) -> &'static str {
    match code {
        OBS_MODULE_SUCCESS => "success",
        sys::MODULE_ERROR => "Generic module error",
        sys::MODULE_FILE_NOT_FOUND => "Module file not found",
        sys::MODULE_MISSING_EXPORTS => "Module missing exports",
        sys::MODULE_INCOMPATIBLE_VER => "Module incompatible version",
        sys::MODULE_HARDCODED_SKIP => "Module skipped by hardcoded rule",
        _ => "unknown error",
    }
}

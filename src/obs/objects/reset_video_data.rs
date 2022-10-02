use crate::obs::sys;
use crate::obs::traits::enum_value::EnumValue;
use napi::bindgen_prelude::ToNapiValue;

/// Data required to reset the video.
#[derive(Clone)]
#[napi(object)]
pub struct ResetVideoData {
    pub graphics_module: GraphicsModule,
    pub fps_num: u32,
    pub fps_den: u32,
    pub base_width: u32,
    pub base_height: u32,
    pub output_width: u32,
    pub output_height: u32,
    pub output_format: VideoFormat,
    pub adapter: u32,
    pub gpu_conversion: bool,
    pub colorspace: VideoColorSpace,
    pub range: VideoRange,
    pub scale_type: ScaleType,
}

unsafe impl Send for ResetVideoData {}

/// The graphics module to use.
#[napi]
pub enum GraphicsModule {
    OpenGL,
    D3D11,
}

impl GraphicsModule {
    pub fn to_string(&self) -> String {
        match self {
            GraphicsModule::OpenGL => "libobs-opengl".to_string(),
            GraphicsModule::D3D11 => "libobs-d3d11".to_string(),
        }
    }
}

/// The video format.
#[napi]
pub enum VideoFormat {
    None,
    I420,
    NV12,
    YVYU,
    YUY2,
    UYVY,
    RGBA,
    BGRA,
    Y800,
    I444,
    I42A,
    YUVA,
    I40A,
    BGR3,
    AYUV,
    I010,
    P010,
    I210,
    I412,
    YA2L,
}

impl EnumValue for VideoFormat {
    fn value(&self) -> i32 {
        match self {
            VideoFormat::None => sys::video_format_VIDEO_FORMAT_NONE,
            VideoFormat::I420 => sys::video_format_VIDEO_FORMAT_I420,
            VideoFormat::NV12 => sys::video_format_VIDEO_FORMAT_NV12,
            VideoFormat::YVYU => sys::video_format_VIDEO_FORMAT_YVYU,
            VideoFormat::YUY2 => sys::video_format_VIDEO_FORMAT_YUY2,
            VideoFormat::UYVY => sys::video_format_VIDEO_FORMAT_UYVY,
            VideoFormat::RGBA => sys::video_format_VIDEO_FORMAT_RGBA,
            VideoFormat::BGRA => sys::video_format_VIDEO_FORMAT_BGRA,
            VideoFormat::Y800 => sys::video_format_VIDEO_FORMAT_Y800,
            VideoFormat::I444 => sys::video_format_VIDEO_FORMAT_I444,
            VideoFormat::I42A => sys::video_format_VIDEO_FORMAT_I42A,
            VideoFormat::YUVA => sys::video_format_VIDEO_FORMAT_YUVA,
            VideoFormat::I40A => sys::video_format_VIDEO_FORMAT_I40A,
            VideoFormat::BGR3 => sys::video_format_VIDEO_FORMAT_BGR3,
            VideoFormat::AYUV => sys::video_format_VIDEO_FORMAT_AYUV,
            VideoFormat::I010 => sys::video_format_VIDEO_FORMAT_I010,
            VideoFormat::P010 => sys::video_format_VIDEO_FORMAT_P010,
            VideoFormat::I210 => sys::video_format_VIDEO_FORMAT_I210,
            VideoFormat::I412 => sys::video_format_VIDEO_FORMAT_I412,
            VideoFormat::YA2L => sys::video_format_VIDEO_FORMAT_YA2L,
        }
    }
}

/// The video color space.
#[napi]
pub enum VideoColorSpace {
    Default,
    CS601,
    CS709,
    SRGB,
    CS2100PQ,
    CS2100HLG,
}

impl EnumValue for VideoColorSpace {
    fn value(&self) -> i32 {
        match self {
            VideoColorSpace::Default => sys::video_colorspace_VIDEO_CS_DEFAULT,
            VideoColorSpace::CS601 => sys::video_colorspace_VIDEO_CS_601,
            VideoColorSpace::CS709 => sys::video_colorspace_VIDEO_CS_709,
            VideoColorSpace::SRGB => sys::video_colorspace_VIDEO_CS_SRGB,
            VideoColorSpace::CS2100PQ => sys::video_colorspace_VIDEO_CS_2100_PQ,
            VideoColorSpace::CS2100HLG => sys::video_colorspace_VIDEO_CS_2100_HLG,
        }
    }
}

/// The video range.
#[napi]
pub enum VideoRange {
    Default,
    Partial,
    Full,
}

impl EnumValue for VideoRange {
    fn value(&self) -> i32 {
        match self {
            VideoRange::Default => sys::video_range_type_VIDEO_RANGE_DEFAULT,
            VideoRange::Partial => sys::video_range_type_VIDEO_RANGE_PARTIAL,
            VideoRange::Full => sys::video_range_type_VIDEO_RANGE_FULL,
        }
    }
}

/// The scale type.
#[napi]
pub enum ScaleType {
    Disable,
    Point,
    Bicubic,
    Bilinear,
    Lanczos,
    Area,
}

impl EnumValue for ScaleType {
    fn value(&self) -> i32 {
        match self {
            ScaleType::Disable => sys::obs_scale_type_OBS_SCALE_DISABLE,
            ScaleType::Point => sys::obs_scale_type_OBS_SCALE_POINT,
            ScaleType::Bicubic => sys::obs_scale_type_OBS_SCALE_BICUBIC,
            ScaleType::Bilinear => sys::obs_scale_type_OBS_SCALE_BILINEAR,
            ScaleType::Lanczos => sys::obs_scale_type_OBS_SCALE_LANCZOS,
            ScaleType::Area => sys::obs_scale_type_OBS_SCALE_AREA,
        }
    }
}

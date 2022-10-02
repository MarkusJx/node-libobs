use crate::obs::sys;
use crate::obs::traits::enum_value::EnumValue;
use napi::bindgen_prelude::ToNapiValue;

/// The data required to reset the audio.
#[napi(object)]
pub struct ResetAudioData {
    pub samples_per_sec: u32,
    pub speakers: SpeakerLayout,
    pub max_buffering_ms: u32,
    pub fixed_buffering: bool,
}

/// OBS audio speaker layout.
#[napi]
pub enum SpeakerLayout {
    Unknown,
    Mono,
    Stereo,
    Layout2point1,
    Layout4point0,
    Layout4point1,
    Layout5point1,
    Layout7point1,
}

impl EnumValue for SpeakerLayout {
    fn value(&self) -> i32 {
        match self {
            SpeakerLayout::Unknown => sys::speaker_layout_SPEAKERS_UNKNOWN,
            SpeakerLayout::Mono => sys::speaker_layout_SPEAKERS_MONO,
            SpeakerLayout::Stereo => sys::speaker_layout_SPEAKERS_STEREO,
            SpeakerLayout::Layout2point1 => sys::speaker_layout_SPEAKERS_2POINT1,
            SpeakerLayout::Layout4point0 => sys::speaker_layout_SPEAKERS_4POINT0,
            SpeakerLayout::Layout4point1 => sys::speaker_layout_SPEAKERS_4POINT1,
            SpeakerLayout::Layout5point1 => sys::speaker_layout_SPEAKERS_5POINT1,
            SpeakerLayout::Layout7point1 => sys::speaker_layout_SPEAKERS_7POINT1,
        }
    }
}

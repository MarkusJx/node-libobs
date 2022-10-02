use crate::obs::data::inner_obs_property::InnerObsProperty;
use crate::obs::sys;
use crate::obs::util::types::ResultType;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum ObsPropertyType {
    Invalid,
    Bool,
    Int,
    Float,
    Text,
    Path,
    List(Vec<String>),
    Color,
    Button,
    Font,
    EditableList,
    FrameRate,
    Group,
    ColorAlpha,
    Unknown,
}

impl TryFrom<&InnerObsProperty> for ObsPropertyType {
    type Error = Box<dyn Error>;

    fn try_from(value: &InnerObsProperty) -> ResultType<Self> {
        Ok(match value.get_property_type() {
            sys::obs_property_type_OBS_PROPERTY_INVALID => ObsPropertyType::Invalid,
            sys::obs_property_type_OBS_PROPERTY_BOOL => ObsPropertyType::Bool,
            sys::obs_property_type_OBS_PROPERTY_INT => ObsPropertyType::Int,
            sys::obs_property_type_OBS_PROPERTY_FLOAT => ObsPropertyType::Float,
            sys::obs_property_type_OBS_PROPERTY_TEXT => ObsPropertyType::Text,
            sys::obs_property_type_OBS_PROPERTY_PATH => ObsPropertyType::Path,
            sys::obs_property_type_OBS_PROPERTY_LIST => {
                ObsPropertyType::List(value.get_list_items()?)
            }
            sys::obs_property_type_OBS_PROPERTY_COLOR => ObsPropertyType::Color,
            sys::obs_property_type_OBS_PROPERTY_BUTTON => ObsPropertyType::Button,
            sys::obs_property_type_OBS_PROPERTY_FONT => ObsPropertyType::Font,
            sys::obs_property_type_OBS_PROPERTY_EDITABLE_LIST => ObsPropertyType::EditableList,
            sys::obs_property_type_OBS_PROPERTY_FRAME_RATE => ObsPropertyType::FrameRate,
            sys::obs_property_type_OBS_PROPERTY_GROUP => ObsPropertyType::Group,
            sys::obs_property_type_OBS_PROPERTY_COLOR_ALPHA => ObsPropertyType::ColorAlpha,
            _ => ObsPropertyType::Unknown,
        })
    }
}

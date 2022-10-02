use napi::{Env, JsFunction, JsNumber};

pub fn is_integer(env: &Env, value: &JsNumber) -> napi::Result<bool> {
    let number = env
        .get_global()?
        .get_named_property::<JsFunction>("Number")?
        .coerce_to_object()?;
    let is_integer: JsFunction = number.get_named_property("isInteger")?;

    Ok(is_integer
        .call(None, &[value])?
        .coerce_to_bool()?
        .get_value()?)
}

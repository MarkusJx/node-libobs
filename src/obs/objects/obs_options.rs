#[napi(object)]
#[derive(Clone)]
pub struct ObsOptions {
    pub binding_path: Option<String>,
    pub shutdown: Option<bool>,
}

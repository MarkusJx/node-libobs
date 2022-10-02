use crate::obs::data::obs_module::ObsModule;

/// An obs module that failed to load.
#[napi(object)]
#[derive(Clone)]
pub struct FailedObsModule {
    /// The info of the failed module.
    pub module: ObsModule,
    /// The error message.
    pub error: String,
}

impl FailedObsModule {
    pub fn new(module: ObsModule, error: String) -> Self {
        Self { module, error }
    }
}

use crate::obs::sys;

pub struct ObsGuard {
    pub(crate) library: sys::Bindings,
    shutdown: bool,
}

impl ObsGuard {
    pub fn new(library: sys::Bindings, shutdown: bool) -> Self {
        Self { library, shutdown }
    }
}

impl Drop for ObsGuard {
    fn drop(&mut self) {
        if self.shutdown {
            unsafe {
                self.library.obs_shutdown();
            }
        }
    }
}

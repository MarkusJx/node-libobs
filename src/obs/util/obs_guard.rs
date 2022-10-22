use crate::obs::sys;

pub struct ObsGuard {
    pub(crate) library: sys::Bindings,
}

impl ObsGuard {
    pub fn new(library: sys::Bindings) -> Self {
        Self { library }
    }
}

impl Drop for ObsGuard {
    fn drop(&mut self) {
        unsafe {
            self.library.obs_shutdown();
        }
    }
}

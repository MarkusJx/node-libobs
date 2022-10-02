use crate::obs::sys;

pub struct ObsGuard {}

impl ObsGuard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Drop for ObsGuard {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping obs");
            sys::obs_shutdown();
        }
    }
}

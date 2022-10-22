use crate::obs::util::obs_guard::ObsGuard;
use std::sync::Arc;

pub(crate) trait FromRaw<T>: Sized {
    fn from_raw(raw: *mut T, guard: Arc<ObsGuard>) -> Self {
        assert!(!raw.is_null(), "Failed to create from null pointer");

        unsafe { Self::from_raw_unchecked(raw, guard) }
    }

    unsafe fn from_raw_unchecked(raw: *mut T, guard: Arc<ObsGuard>) -> Self;
}

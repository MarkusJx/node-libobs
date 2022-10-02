pub trait Raw<T> {
    unsafe fn raw(&self) -> *mut T;
}

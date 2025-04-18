use std::mem::MaybeUninit;
use std::ptr::NonNull;

use objc2_core_foundation::{CFRetained, Type};

pub use self::error::*;
pub use self::ui_element::*;
pub use self::value::*;

pub mod attribute;
mod error;
pub mod notification;
mod ui_element;
mod value;

// see https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1

unsafe fn cf_call_get<T, E, F>(f: F) -> Result<T, E>
where
    F: FnOnce(NonNull<T>) -> Result<(), E>,
{
    let mut out = MaybeUninit::<T>::uninit();
    let ptr = unsafe { NonNull::new_unchecked(out.as_mut_ptr()) };
    match f(ptr) {
        Ok(_) => {
            // SAFETY: Caller guarantees that a value will be set on success.
            Ok(unsafe { out.assume_init() })
        }
        Err(err) => Err(err),
    }
}

unsafe fn cf_call_owned<T, E, F>(f: F) -> Result<CFRetained<T>, E>
where
    T: ?Sized + Type,
    F: FnOnce(NonNull<*const T>) -> Result<(), E>,
{
    let mut out = MaybeUninit::zeroed();
    let ptr = unsafe { NonNull::new_unchecked(out.as_mut_ptr()) };
    match f(ptr) {
        Ok(_) => {
            // SAFETY: Caller guarantees that a valid object pointer will be set
            // on success.
            let ptr = unsafe { NonNull::new(out.assume_init().cast_mut()).unwrap() };

            // SAFETY: Caller guarantees that the returned object pointer
            // follows the Create rule (i.e. already has +1 retained count).
            let retained = unsafe { CFRetained::from_raw(ptr) };

            Ok(retained)
        }
        Err(err) => Err(err),
    }
}

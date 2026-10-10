// TODO @joey probably just inline all of these hwen im not being lazy

use core::{ffi::c_char, slice};
use std::ffi::CStr;

macro_rules! owned_handle {
    ($name:ident, $raw:ty, $drop:path) => {
        pub struct $name<'engine> {
            pub(crate) raw: core::ptr::NonNull<$raw>,
            _engine: core::marker::PhantomData<&'engine $crate::Engine>,
        }

        impl $name<'_> {
            #[inline]
            pub fn as_raw(&self) -> *mut $raw {
                self.raw.as_ptr()
            }

            #[inline]
            pub fn into_raw(self) -> *mut $raw {
                let this = core::mem::ManuallyDrop::new(self);
                this.raw.as_ptr()
            }

            /// # Safety
            /// `raw` must be live, owned, and allocated by the same active engine
            #[inline]
            pub unsafe fn from_raw(raw: *mut $raw) -> $crate::Result<Self> {
                Ok(Self {
                    raw: core::ptr::NonNull::new(raw)
                        .ok_or($crate::Error::Null(concat!("adopting ", stringify!($name))))?,
                    _engine: core::marker::PhantomData,
                })
            }
        }

        impl Drop for $name<'_> {
            fn drop(&mut self) {
                unsafe { $drop(self.raw.as_ptr()) }
            }
        }
    };
}

pub(crate) use owned_handle;

#[inline]
pub(crate) fn nonneg(value: i32) -> usize {
    usize::try_from(value).unwrap_or(0)
}

#[inline]
pub(crate) fn nonneg_product(a: i32, b: i32) -> usize {
    nonneg(a).saturating_mul(nonneg(b))
}

#[inline]
pub(crate) fn slice_from_raw_mut<'a, T>(raw: *mut T, len: usize) -> Option<&'a mut [T]> {
    if len == 0 {
        Some(&mut [])
    } else if raw.is_null() {
        None
    } else {
        Some(unsafe { slice::from_raw_parts_mut(raw, len) })
    }
}

#[inline]
pub(crate) fn slice_from_raw<'a, T>(raw: *const T, len: usize) -> Option<&'a [T]> {
    if len == 0 {
        Some(&[])
    } else if raw.is_null() {
        None
    } else {
        Some(unsafe { slice::from_raw_parts(raw, len) })
    }
}

#[inline]
pub(crate) fn cstr<'a>(raw: *const c_char) -> &'a CStr {
    if raw.is_null() {
        c""
    } else {
        unsafe { CStr::from_ptr(raw) }
    }
}

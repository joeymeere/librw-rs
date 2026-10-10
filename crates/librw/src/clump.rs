use core::{marker::PhantomData, ptr, ptr::NonNull};

use librw_sys as sys;

use crate::util::{nonneg, owned_handle};
use crate::{Engine, Error, Geometry, GeometryRef, Result, Stream};

owned_handle!(Atomic, sys::rw_Atomic, sys::librw_atomic_destroy);
owned_handle!(Clump, sys::rw_Clump, sys::librw_clump_destroy);

impl<'engine> Atomic<'engine> {
    pub fn new(_engine: &'engine Engine) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_atomic_create()) }
    }
    pub fn set_geometry(&mut self, geometry: Option<&Geometry<'engine>>) {
        unsafe {
            sys::librw_atomic_set_geometry(
                self.raw.as_ptr(),
                geometry.map_or(ptr::null_mut(), |v| v.raw.as_ptr()),
                0,
            )
        }
    }
    pub fn geometry(&self) -> Option<GeometryRef<'_>> {
        NonNull::new(unsafe { sys::librw_atomic_geometry(self.raw.as_ptr()) }).map(|raw| {
            GeometryRef {
                raw,
                _owner: PhantomData,
            }
        })
    }
    pub fn flags(&self) -> u32 {
        unsafe { sys::librw_atomic_flags(self.raw.as_ptr()) }
    }
    pub fn set_flags(&mut self, flags: u32) {
        unsafe { sys::librw_atomic_set_flags(self.raw.as_ptr(), flags) }
    }
    pub fn render(&mut self) {
        unsafe { sys::librw_atomic_render(self.raw.as_ptr()) }
    }
}

pub struct AtomicRef<'a> {
    pub(crate) raw: NonNull<sys::rw_Atomic>,
    pub(crate) _owner: PhantomData<&'a mut sys::rw_Atomic>,
}
impl AtomicRef<'_> {
    pub fn as_raw(&self) -> *mut sys::rw_Atomic {
        self.raw.as_ptr()
    }
    pub fn flags(&self) -> u32 {
        unsafe { sys::librw_atomic_flags(self.raw.as_ptr()) }
    }
    pub fn geometry(&self) -> Option<GeometryRef<'_>> {
        NonNull::new(unsafe { sys::librw_atomic_geometry(self.raw.as_ptr()) }).map(|raw| {
            GeometryRef {
                raw,
                _owner: PhantomData,
            }
        })
    }
    pub fn set_flags(&mut self, flags: u32) {
        unsafe { sys::librw_atomic_set_flags(self.raw.as_ptr(), flags) }
    }
    pub fn render(&mut self) {
        unsafe { sys::librw_atomic_render(self.raw.as_ptr()) }
    }
}

impl<'engine> Clump<'engine> {
    pub fn new(_engine: &'engine Engine) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_clump_create()) }
    }
    pub fn read(_engine: &'engine Engine, stream: &mut impl Stream) -> Result<Self> {
        stream.find_chunk(sys::RW_ID_CLUMP)?;
        unsafe { Self::from_raw(sys::librw_clump_read(stream.as_raw_stream())) }
    }
    pub fn read_payload(_engine: &'engine Engine, stream: &mut impl Stream) -> Result<Self> {
        unsafe { Self::from_raw(sys::librw_clump_read(stream.as_raw_stream())) }
    }
    pub fn write(&mut self, stream: &mut impl Stream) -> Result<()> {
        if unsafe { sys::librw_clump_write(self.raw.as_ptr(), stream.as_raw_stream()) } == 0 {
            Err(Error::Engine("writing a clump"))
        } else {
            Ok(())
        }
    }
    pub fn add_atomic(&mut self, atomic: Atomic<'engine>) {
        unsafe { sys::librw_clump_add_atomic(self.raw.as_ptr(), atomic.into_raw()) }
    }
    pub fn atomic_count(&self) -> usize {
        nonneg(unsafe { sys::librw_clump_atomic_count(self.raw.as_ptr()) })
    }
    pub fn atomic(&mut self, index: usize) -> Option<AtomicRef<'_>> {
        NonNull::new(unsafe { sys::librw_clump_atomic(self.raw.as_ptr(), index as i32) }).map(
            |raw| AtomicRef {
                raw,
                _owner: PhantomData,
            },
        )
    }
    pub fn atomics(&mut self) -> Atomics<'_> {
        let len = self.atomic_count();
        Atomics {
            clump: self.raw,
            index: 0,
            len,
            _owner: PhantomData,
        }
    }
    pub fn render(&mut self) {
        unsafe { sys::librw_clump_render(self.raw.as_ptr()) }
    }
}
pub struct Atomics<'a> {
    clump: NonNull<sys::rw_Clump>,
    index: usize,
    len: usize,
    _owner: PhantomData<&'a mut sys::rw_Clump>,
}
impl<'a> Iterator for Atomics<'a> {
    type Item = AtomicRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let raw = NonNull::new(unsafe {
            sys::librw_clump_atomic(self.clump.as_ptr(), self.index as i32)
        })?;
        self.index += 1;
        Some(AtomicRef {
            raw,
            _owner: PhantomData,
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len - self.index;
        (n, Some(n))
    }
}
impl ExactSizeIterator for Atomics<'_> {}

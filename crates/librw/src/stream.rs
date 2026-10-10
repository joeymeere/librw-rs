use core::{marker::PhantomData, ptr::NonNull};
use std::{
    ffi::{CStr, CString},
    path::Path,
};

use librw_sys as sys;

use crate::{Engine, Error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileMode {
    Read,
    Write,
}

impl Into<&'static CStr> for FileMode {
    fn into(self) -> &'static CStr {
        match self {
            Self::Read => c"rb",
            Self::Write => c"wb",
        }
    }
}

pub trait Stream {
    fn as_raw_stream(&mut self) -> *mut sys::rw_Stream;

    fn read(&mut self, output: &mut [u8]) -> Result<usize> {
        let len = u32::try_from(output.len()).map_err(|_| Error::LengthOverflow)?;
        Ok(
            unsafe { sys::librw_stream_read(self.as_raw_stream(), output.as_mut_ptr().cast(), len) }
                as usize,
        )
    }

    fn write(&mut self, input: &[u8]) -> Result<usize> {
        let len = u32::try_from(input.len()).map_err(|_| Error::LengthOverflow)?;
        Ok(
            unsafe { sys::librw_stream_write(self.as_raw_stream(), input.as_ptr().cast(), len) }
                as usize,
        )
    }

    fn seek(&mut self, offset: i32, whence: SeekFrom) {
        unsafe { sys::librw_stream_seek(self.as_raw_stream(), offset, whence as i32) }
    }

    fn position(&mut self) -> u32 {
        unsafe { sys::librw_stream_tell(self.as_raw_stream()) }
    }

    fn is_eof(&mut self) -> bool {
        unsafe { sys::librw_stream_eof(self.as_raw_stream()) != 0 }
    }

    fn find_chunk(&mut self, kind: u32) -> Result<Chunk> {
        let (mut length, mut version) = (0, 0);
        let found = unsafe {
            sys::librw_stream_find_chunk(self.as_raw_stream(), kind, &mut length, &mut version)
        };
        if found == 0 {
            Err(Error::ChunkNotFound(kind))
        } else {
            Ok(Chunk {
                kind,
                length,
                version,
            })
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeekFrom {
    Start = 0,
    Current = 1,
    End = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub kind: u32,
    pub length: u32,
    pub version: u32,
}

pub struct MemoryStream<'engine, 'data> {
    raw: NonNull<sys::rw_Stream>,
    _engine: PhantomData<&'engine Engine>,
    _data: PhantomData<&'data mut [u8]>,
}

impl<'engine, 'data> MemoryStream<'engine, 'data> {
    pub(crate) fn new(
        _engine: &'engine Engine,
        bytes: &'data mut [u8],
        initial_len: usize,
    ) -> Result<Self> {
        let capacity = u32::try_from(bytes.len()).map_err(|_| Error::LengthOverflow)?;
        let len = u32::try_from(initial_len).map_err(|_| Error::LengthOverflow)?;
        let raw = unsafe { sys::librw_stream_memory_new(bytes.as_mut_ptr(), len, capacity) };
        Ok(Self {
            raw: NonNull::new(raw).ok_or(Error::Null("opening a memory stream"))?,
            _engine: PhantomData,
            _data: PhantomData,
        })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { sys::librw_stream_memory_len(self.raw.as_ptr()) as usize }
    }
}

impl Stream for MemoryStream<'_, '_> {
    fn as_raw_stream(&mut self) -> *mut sys::rw_Stream {
        self.raw.as_ptr()
    }
}

impl Drop for MemoryStream<'_, '_> {
    fn drop(&mut self) {
        unsafe { sys::librw_stream_memory_delete(self.raw.as_ptr()) }
    }
}

pub struct FileStream<'engine> {
    raw: NonNull<sys::rw_Stream>,
    _engine: PhantomData<&'engine Engine>,
}

impl<'engine> FileStream<'engine> {
    pub(crate) fn open(_engine: &'engine Engine, path: &Path, mode: FileMode) -> Result<Self> {
        let path = CString::new(path.to_str().ok_or(Error::NonUtf8Path)?)?;
        let md: &'static CStr = mode.into();

        let raw = unsafe { sys::librw_stream_file_new(path.as_ptr(), md.as_ptr()) };
        Ok(Self {
            raw: NonNull::new(raw).ok_or(Error::Null("opening a file stream"))?,
            _engine: PhantomData,
        })
    }
}

impl Stream for FileStream<'_> {
    fn as_raw_stream(&mut self) -> *mut sys::rw_Stream {
        self.raw.as_ptr()
    }
}

impl Drop for FileStream<'_> {
    fn drop(&mut self) {
        unsafe { sys::librw_stream_file_delete(self.raw.as_ptr()) }
    }
}

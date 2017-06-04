extern crate libc;

use libc::{c_char, size_t};
use std::ffi::CStr;
use std::fs::File;
use std::ptr;
use std::slice;
use std::io::{Cursor, Read};

pub struct AudioStream<R: ?Sized> {
    stream: R,
}

impl<R: Read> AudioStream<R> {
    pub fn new(stream: R) -> AudioStream<R> {
        AudioStream { stream: stream }
    }
}

impl<R: ?Sized + Read> AudioStream<R> {
    pub fn samples<'a>(&'a mut self) -> Samples<'a, R> {
        Samples::new(self)
    }
}

pub struct Samples<'a, R: 'a + Read + ?Sized> {
    stream: &'a mut AudioStream<R>,
}

impl<'a, R: 'a + Read + ?Sized> Samples<'a, R> {
    pub fn new(stream: &'a mut AudioStream<R>) -> Samples<'a, R> {
        Samples { stream: stream }
    }
}

impl<'a, R: 'a + Read + ?Sized> Iterator for Samples<'a, R> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        unimplemented!();
    }
}

type audiostream_t = Box<AudioStream<Read>>;

fn make_audiostream<R: Read + 'static>(stream: R) -> *mut audiostream_t {
    let stream = AudioStream::new(stream);
    let stream: Box<AudioStream<Read>> = Box::new(stream);
    let stream = Box::new(stream);
    Box::into_raw(stream)
}

#[no_mangle]
pub extern "C" fn rs_audiostream_from_ptr(p: *const u8, len: size_t) -> *mut audiostream_t {
    if p.is_null() {
        return ptr::null_mut();
    }
    let bytes = unsafe { slice::from_raw_parts(p, len as usize) };
    let bytes: Vec<u8> = bytes.to_owned();
    let bytes = Cursor::new(bytes);

    make_audiostream(bytes)
}

#[no_mangle]
pub extern "C" fn rs_audiostream_from_file(file: *const c_char) -> *mut audiostream_t {
    if file.is_null() {
        return ptr::null_mut();
    }
    let file = unsafe { CStr::from_ptr(file) };
    let file = if let Ok(f) = file.to_str() {
        f
    } else {
        return ptr::null_mut();
    };

    let file = if let Ok(f) = File::open(file) {
        f
    } else {
        return ptr::null_mut();
    };

    make_audiostream(file)
}

#[no_mangle]
pub extern "C" fn rs_destroy_audiostream(stream: *mut audiostream_t) {
    unsafe {
        let _stream = Box::from_raw(stream);
    }
}

type samples_t = Samples<'static, Read>;

#[no_mangle]
pub extern "C" fn rs_new_samples(stream: *mut audiostream_t) -> *mut samples_t {
    if stream.is_null() {
        return ptr::null_mut();
    }
    let stream = unsafe { &mut *stream };

    let samples = stream.samples();
    let samples = Box::new(samples);
    let samples = Box::into_raw(samples);

    samples as *mut samples_t
}

#[no_mangle]
pub extern "C" fn rs_destroy_samples(samples: *mut samples_t) {
    unsafe {
        let _samples = Box::from_raw(samples);
    }
}
#[no_mangle]
pub extern "C" fn rs_samples_next(samples: *mut samples_t, sample: *mut f32) -> i32 {
    if samples.is_null() {
        return 1;
    }
    let samples = unsafe { &mut *samples };
    if let Some(s) = samples.next() {
        unsafe {
            *sample = s;
        }
        return 0;
    } else {
        return 1;
    }
}

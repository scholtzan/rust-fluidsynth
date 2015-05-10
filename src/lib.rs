#![feature(libc)]
#![feature(old_io)]

extern crate libc;
use libc::{c_int, c_char};
use std::ffi::{CString};
mod ffi;

pub mod modulator;
pub mod settings;
pub mod synth;
pub mod audio;
pub mod midi;
pub mod event;
pub mod seq;
pub mod voice;
pub mod ramsfont;

pub fn is_soundfont(filename: &str) -> bool {
    unsafe { 
        let name = CString::new(filename).unwrap().as_ptr();
        ffi::fluid_is_soundfont(name) != 0
    }
}

pub fn is_midifile(filename: &str) -> bool {
    unsafe { 
        let name = CString::new(filename).unwrap().as_ptr();
        ffi::fluid_is_midifile(name) != 0
    }
}



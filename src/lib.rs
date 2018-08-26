extern crate libc;
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
pub mod gen;
pub mod log;
pub mod sfont;

pub fn is_soundfont(filename: &str) -> bool {
    let name = CString::new(filename).unwrap();
    unsafe {
        ffi::fluid_is_soundfont(name.as_ptr()) != 0
    }
}

pub fn is_midifile(filename: &str) -> bool {
    let name = CString::new(filename).unwrap();
    unsafe { 
        ffi::fluid_is_midifile(name.as_ptr()) != 0
    }
}



extern crate libc;
use libc::{c_int};
use ffi::*;

pub fn mod_new() -> *mut fluid_mod_t {
    unsafe {fluid_mod_new()}
}

pub fn mod_delete(modulator: *mut fluid_mod_t) {
    unsafe {fluid_mod_delete(modulator)}
}

pub fn mod_set_source1(modulator: *mut fluid_mod_t, src: i32, flags: i32) {
    unsafe {fluid_mod_set_source1(modulator, src as c_int, flags as c_int)}
}

pub fn mod_set_source2(modulator: *mut fluid_mod_t, src: i32, flags: i32) {
    unsafe {fluid_mod_set_source2(modulator, src as c_int, flags as c_int)}
}



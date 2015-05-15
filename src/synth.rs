extern crate libc;
use libc::{c_int, c_char};
use ffi::*;
use settings::*;

pub struct Synth {
    c_fluid_synth: *mut fluid_synth_t
}

impl Synth {
    pub fn new(settings: &mut Settings) -> Synth {
        unsafe {
            Synth {
                c_fluid_synth: new_fluid_synth(settings.to_raw()) 
            }
        }
    }

    pub fn sfload(&self, filename: &str, reset_presets: i32) -> i32 {
        unsafe {
            let bytes = String::from_str(filename).into_bytes() + b"\0";
            let mut cchars = bytes.map_in_place(|b| b as c_char);
            let name: *mut c_char = cchars.as_mut_ptr();

            let res = fluid_synth_sfload(self.c_fluid_synth, name, reset_presets as c_int);
            res as i32
        }                    
    }

    pub fn noteon(&self, chan: i32, key: i32, vel: i32) -> i32 {
        unsafe {
            let res = fluid_synth_noteon(self.c_fluid_synth, chan as c_int, key as c_int, vel as c_int);
            res as i32
        }                          
    }

    pub fn noteoff(&self, chan: i32, key: i32) -> i32 {
        unsafe {
            let res = fluid_synth_noteoff(self.c_fluid_synth, chan as c_int, key as c_int);
            res as i32
        }                          
    }

    pub fn to_raw(&self) -> *mut fluid_synth_t {
        self.c_fluid_synth
    }
}

impl Drop for Synth {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_synth(self.c_fluid_synth);
        }
    }
}


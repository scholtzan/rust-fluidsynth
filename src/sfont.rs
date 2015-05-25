extern crate libc;
use ffi::*;

pub struct SoundFont {
    c_fluid_sfont: *mut fluid_sfont_t
}

impl SoundFont {
    pub fn from_raw(sfont: *mut fluid_sfont_t) -> SoundFont {
        SoundFont {
            c_fluid_sfont: sfont
        }
    }

    pub fn to_raw(&self) -> *mut fluid_sfont_t {
        self.c_fluid_sfont
    }
}

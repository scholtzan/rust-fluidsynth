extern crate libc;
use libc::{c_int, c_double};
use ffi::*;
use gen::*;
use std::mem::*;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum ModulatorSource {
    None = 0,
    Velocity = 2, 
    Key = 3,
    KeyPressure = 10,
    ChannelPressure = 13,
    PitchWheel = 14,
    PitchWheelSens = 16,
}


#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum ModulatorFlags {
    Positive = 0,
    Negative, 
    Unipolar,
    Bipolar,
    Linear,
    Concave,
    Convex,
    Switch,
    GC,
    CC,
}

pub struct Modulator {
    c_fluid_mod: *mut fluid_mod_t
}

impl Modulator {
    pub fn new() -> Modulator {
        unsafe {
            Modulator {
                c_fluid_mod: fluid_mod_new()
            }
        }
    }

    pub fn set_source1(&self, src: ModulatorSource, flags: ModulatorFlags) {
        unsafe {
            fluid_mod_set_source1(self.to_raw(), src as c_int, flags as c_int)
        }
    }

    pub fn set_source2(&self, src: ModulatorSource, flags: ModulatorFlags) {
        unsafe {
            fluid_mod_set_source2(self.to_raw(), src as c_int, flags as c_int)
        }
    }

    pub fn set_dest(&self, dest: GenType) {
        unsafe {
            fluid_mod_set_dest(self.to_raw(), dest as c_int)
        }
    }

    pub fn set_amount(&self, amount: f64) {
        unsafe {
            fluid_mod_set_amount(self.to_raw(), amount as c_double)
        }
    }

    pub fn get_source1(&self) -> ModulatorSource {
        unsafe {
            transmute(fluid_mod_get_source1(self.to_raw()))
        }
    }

    pub fn get_source2(&self) -> ModulatorSource {
        unsafe {
            transmute(fluid_mod_get_source2(self.to_raw()))
        }
    }

    pub fn get_flags1(&self) -> ModulatorFlags {
        unsafe {
            transmute(fluid_mod_get_flags1(self.to_raw()))
        }
    }

    pub fn get_flags2(&self) -> ModulatorFlags {
        unsafe {
            transmute(fluid_mod_get_flags2(self.to_raw()))
        }
    }

    pub fn get_dest(&self) -> GenType {
        unsafe {
            transmute(fluid_mod_get_dest(self.to_raw()))
        }
    }

    pub fn get_amount(&self) -> f64 {
        unsafe {
            fluid_mod_get_amount(self.to_raw())
        }
    }

    pub fn test_identity(&self, mod2: &Modulator) -> bool {
        unsafe {
            fluid_mod_test_identity(self.to_raw(), mod2.to_raw()) != 0
        }
    }

    pub fn to_raw(&self) -> *mut fluid_mod_t {
        self.c_fluid_mod
    }
}


impl Drop for Modulator {
    fn drop(&mut self) -> () {
        unsafe {
            fluid_mod_delete(self.c_fluid_mod);
        }
    }
}


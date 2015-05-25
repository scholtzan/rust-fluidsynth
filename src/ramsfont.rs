extern crate libc;
use ffi::*;
use libc::{c_int, c_uint, c_short, c_float};
use std::ffi::{CString};


pub struct RAMSoundFont {
    _c_fluid_sfont: *mut fluid_sfont_t,
    c_fluid_ramsfont: *mut fluid_ramsfont_t
}

impl RAMSoundFont {
    pub fn new() -> RAMSoundFont {
        unsafe {
            let font: *mut fluid_sfont_t = fluid_ramsfont_create_sfont();
            RAMSoundFont {
                _c_fluid_sfont: font,
                c_fluid_ramsfont: (*font).data as *mut fluid_ramsfont_t
            }
        }
    }

    pub fn set_name(&self, name: &str) -> i32 {
        unsafe {
            fluid_ramsfont_set_name(self.to_raw(), CString::new(name).unwrap().as_ptr())
        }
    } 

    pub fn add_izone(&self, bank: u32, prog_num: u32, sample: &RAMSample, low_key: i32, high_key: i32) -> i32 {
        unsafe {
            fluid_ramsfont_add_izone(self.to_raw(), bank as c_uint, prog_num as c_uint, sample.to_raw(), low_key as c_int, high_key as c_int)
        }
    } 

    pub fn remove_izone(&self, bank: u32, prog_num: u32, sample: &RAMSample) -> i32 {
        unsafe {
            fluid_ramsfont_remove_izone(self.to_raw(), bank as c_uint, prog_num as c_uint, sample.to_raw())
        }
    } 

    pub fn izone_set_gen(&self, bank: u32, prog_num: u32, sample: &RAMSample, gen_type: i32, value: f32) -> i32 {
        unsafe {
            fluid_ramsfont_izone_set_gen(self.to_raw(), bank as c_uint, prog_num as c_uint, sample.to_raw(), gen_type as c_int, value as c_float)
        }
    } 

    pub fn izone_set_loop(&self, bank: u32, prog_num: u32, sample: &RAMSample, on: bool, loopstart: f32, loopend: f32) -> i32 {
        unsafe {
            fluid_ramsfont_izone_set_loop(self.to_raw(), bank as c_uint, prog_num as c_uint, sample.to_raw(), on as c_int, loopstart as c_float, loopend as c_float)
        }
    } 

    pub fn to_raw(&self) -> *mut fluid_ramsfont_t {
        self.c_fluid_ramsfont
    }
}


pub struct RAMSample {
    c_fluid_sample: *mut fluid_sample_t
}

impl RAMSample {
    pub fn new() -> RAMSample {
        unsafe {
            RAMSample {
                c_fluid_sample: new_fluid_ramsample()
            }
        }
    }

    pub fn set_name(&self, name: &str) -> i32 {
        unsafe {
            fluid_sample_set_name(self.to_raw(), CString::new(name).unwrap().as_ptr())
        }
    }

    pub fn set_sound_data(&self, data: *mut i16, nbframes: u32, copy_data: bool, rootkey: i32) -> i32 {
        unsafe {
            fluid_sample_set_sound_data(self.to_raw(), data as *mut c_short, nbframes as c_uint, copy_data as c_short, rootkey as c_int)
        }
    }

    pub fn to_raw(&self) -> *mut fluid_sample_t {
        self.c_fluid_sample
    }
}

impl Drop for RAMSample {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_ramsample(self.to_raw()); 
        }
    }
}


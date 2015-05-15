extern crate libc;
use libc::{c_void, c_int, c_float};
use ffi::*;
use synth::*;
use settings::*;
use event::*;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum VoiceAddMod {
    Overwrite = 0,
    Add, 
    Default,
}



// TODO

/*pub struct Voice {
    c_fluid_voice: *mut fluid_voice_t
}

impl Voice {
    pub fn new() -> Voice {
        Voice {
            c_fluid_voice: *mut fluid_voice_t
        }
    }
}*/


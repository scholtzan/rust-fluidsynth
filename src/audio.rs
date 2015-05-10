extern crate libc;
use ffi::*;
use settings::*;
use synth::*;

pub struct AudioDriver {
    c_fluid_audio_driver: *mut fluid_audio_driver_t,
}

impl AudioDriver {
    pub fn new(settings: &mut Settings, synth: &mut Synth) -> AudioDriver {
        unsafe {
            AudioDriver {
                c_fluid_audio_driver: new_fluid_audio_driver(settings.to_raw(), synth.to_raw())
            }
        }
    }

    // TODO
    /*pub fn new2(settings: &mut Settings, func: ) {

    }*/

    pub fn to_raw(&mut self) -> *mut fluid_audio_driver_t {
        self.c_fluid_audio_driver
    }
}

impl Drop for AudioDriver {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_audio_driver(self.c_fluid_audio_driver); 
        }
    }
}


pub struct FileRenderer {
    c_fluid_file_renderer: *mut fluid_file_renderer_t
}

impl FileRenderer {
    pub fn new(synth: &mut Synth) -> FileRenderer {
        unsafe {
            FileRenderer {
                c_fluid_file_renderer: new_fluid_file_renderer(synth.to_raw())
            }
        }
    }

    pub fn to_raw(&self) -> *mut fluid_file_renderer_t {
        self.c_fluid_file_renderer
    }

    pub fn process_block(&self) -> i32 {
        unsafe {
            fluid_file_renderer_process_block(self.to_raw())
        }
    }
}

impl Drop for FileRenderer {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_file_renderer(self.c_fluid_file_renderer); 
        }
    }
}


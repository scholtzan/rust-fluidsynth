extern crate libc;
use libc::{c_int, c_uint, c_short, c_char, c_void, c_double};
use ffi::*;
use synth::*;
use event::*;
use std::ffi::{CString, CStr};
use std::str;


#[derive(Clone)]
pub struct Sequencer {
    c_fluid_sequencer: *mut fluid_sequencer_t
}

impl Sequencer {
    pub fn new() -> Sequencer {
        unsafe {
            Sequencer {
                c_fluid_sequencer: new_fluid_sequencer()
            }
        }
    }

    pub fn from_raw(raw_seq: *mut fluid_sequencer_t) -> Sequencer {
        Sequencer {
            c_fluid_sequencer: raw_seq
        }
    }

    pub fn new2(use_system_timer: i32) -> Sequencer {
        unsafe {
            Sequencer {
                c_fluid_sequencer: new_fluid_sequencer2(use_system_timer as c_int)
            }
        }
    }

    pub fn get_use_system_timer(&self) -> bool {
        unsafe {
            fluid_sequencer_get_use_system_timer(self.to_raw()) != 0
        }
    }

    pub fn to_raw(&self) -> *mut fluid_sequencer_t {
        self.c_fluid_sequencer
    }

    pub fn register_fluidsynth(&self, synth: &mut Synth) -> i16 {
        unsafe {
            fluid_sequencer_register_fluidsynth(self.to_raw(), synth.to_raw())
        }
    }

    pub fn register_client<T: Fn(u32, Event, Sequencer)>(&self, name: &str, callback: T) -> i16 {
        unsafe {
            // get name
            let name = CString::new(name).unwrap().as_ptr();

            let user_data = &callback as *const _ as *mut c_void;
            
            return fluid_sequencer_register_client(self.c_fluid_sequencer, name, 
                                                   event_callback_wrapper::<T>, user_data) as i16
        }

        fn event_callback_wrapper<T>(closure: *mut c_void, time: c_uint, event: *mut fluid_event_t, seq: *mut fluid_sequencer_t)
            where T: Fn(u32, Event, Sequencer) {
            let closure = closure as *mut T;

            unsafe {
                (*closure)(time as u32, Event::from_raw(event), Sequencer::from_raw(seq));
            }
        }
    }

    pub fn unregister_client(&self, id: i16) {
        unsafe {
            fluid_sequencer_unregister_client(self.to_raw(), id as c_short);
        }
    }

    pub fn count_clients(&self) -> i32 {
        unsafe {
            fluid_sequencer_count_clients(self.to_raw())
        }
    }

    pub fn get_client_id(&self, index: i32) -> i16 {
        unsafe {
            fluid_sequencer_get_client_id(self.to_raw(), index as c_int)
        }
    }

    pub fn get_client_name(&self, index: i32) -> &str {
        unsafe {
            let raw_name = fluid_sequencer_get_client_name(self.to_raw(), index as i32);
            let slice = CStr::from_ptr(raw_name);
            str::from_utf8(slice.to_bytes()).unwrap()
        }
    }

    pub fn client_is_dest(&self, index: i32) -> bool {
        unsafe {
            fluid_sequencer_client_is_dest(self.to_raw(), index as i32) != 0
        }
    }

    pub fn process(&self, msec: u32) {
        unsafe {
            fluid_sequencer_process(self.to_raw(), msec as u32);
        }
    }

    pub fn send_now(&self, event: &mut Event) {
        unsafe {
            fluid_sequencer_send_now(self.c_fluid_sequencer, event.to_raw());
        }
    }

    pub fn send_at(&self, event: &mut Event, time: u32, absolute: i32) -> i32 {
        unsafe {
            fluid_sequencer_send_at(self.c_fluid_sequencer, event.to_raw(), time as c_uint, absolute as c_int)
        }
    }

    pub fn remove_events(&self, source: i16, destination: i16, event_type: i32) {
        unsafe {
            fluid_sequencer_remove_events(self.c_fluid_sequencer, source as c_short, destination as c_short, event_type as c_int);
        }
    }

    pub fn get_tick(&self) -> u32 {
        unsafe {
            fluid_sequencer_get_tick(self.c_fluid_sequencer)
        }
    }

    pub fn set_time_scale(&self, scale: f64) {
        unsafe {
            fluid_sequencer_set_time_scale(self.to_raw(), scale as c_double)
        }
    }

    pub fn get_time_scale(&self) -> f64 {
        unsafe {
            fluid_sequencer_get_time_scale(self.to_raw())
        }
    }
}

impl Drop for Sequencer {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_sequencer(self.c_fluid_sequencer); 
        }
    }
}


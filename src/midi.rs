extern crate libc;
use libc::{c_void, c_int, c_float};
use ffi::*;
use settings::*;
use event::*;

pub struct MidiRouter {
    c_fluid_midi_router: *mut fluid_midi_router_t
}

impl MidiRouter {
    pub fn new<T: Fn(MidiEvent) -> i32>(settings: &mut Settings, callback: T) -> MidiRouter {
        unsafe {
            let user_data = &callback as *const _ as *mut c_void;
            let router = new_fluid_midi_router(settings.to_raw(), midi_router_callback_wrapper::<T>, user_data);

            return MidiRouter {
                c_fluid_midi_router: router
            }
        }

        fn midi_router_callback_wrapper<T>(closure: *mut c_void, event: *mut fluid_midi_event_t) -> i32
            where T: Fn(MidiEvent) -> i32 {
            let closure = closure as *mut T;

            unsafe {
                (*closure)(MidiEvent::from_raw(event))
            }
        }
    }

    pub fn add_rule(&self, rule: &mut MidiRouterRule, rule_type: MidiRouterRuleType) -> i32 {
        unsafe { 
            fluid_midi_router_add_rule(self.to_raw(), rule.to_raw(), rule_type as c_int) 
        }
    }

    pub fn clear_rules(&self) -> i32 {
        unsafe { 
            fluid_midi_router_clear_rules(self.to_raw()) 
        }
    }

    pub fn to_raw(&self) -> *mut fluid_midi_router_t {
        self.c_fluid_midi_router
    }
}

impl Drop for MidiRouter {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_midi_router(self.c_fluid_midi_router);
        }
    }
}

pub enum MidiRouterRuleType {
    Note = 0,
    CC,
    ProgChange,
    PitchBend,
    ChannelPressure,
    KeyPressure,
    Count,
}


pub struct MidiRouterRule {
    c_fluid_midi_router_rule: *mut fluid_midi_router_rule_t
}

impl MidiRouterRule {
    pub fn new() -> MidiRouterRule {
        unsafe {
            MidiRouterRule {
                c_fluid_midi_router_rule: new_fluid_midi_router_rule() 
            }
        }
    }

    pub fn set_chan(&self, min: i32, max: i32, mul: f32, add: i32) {
        unsafe { 
            fluid_midi_router_rule_set_chan(self.to_raw(), min as c_int, max as c_int, mul as c_float, add as c_int) 
        }
    }

    pub fn set_param1(&self, min: i32, max: i32, mul: f32, add: i32) {
        unsafe { 
            fluid_midi_router_rule_set_param1(self.to_raw(), min as c_int, max as c_int, mul as c_float, add as c_int) 
        }
    }

    pub fn set_param2(&self, min: i32, max: i32, mul: f32, add: i32) {
        unsafe { 
            fluid_midi_router_rule_set_param2(self.to_raw(), min as c_int, max as c_int, mul as c_float, add as c_int) 
        }
    }

    pub fn to_raw(&self) -> *mut fluid_midi_router_rule_t {
        self.c_fluid_midi_router_rule
    }
}

impl Drop for MidiRouterRule {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_midi_router_rule(self.c_fluid_midi_router_rule);
        }
    }
}



pub struct MidiDriver {
    c_fluid_midi_driver: *mut fluid_midi_driver_t,
}


impl MidiDriver {
    pub fn new<T: Fn(MidiEvent) -> i32>(settings: &mut Settings, callback: T) -> MidiDriver {
        unsafe {
            let user_data = &callback as *const _ as *mut c_void;
            let router = new_fluid_midi_driver(settings.to_raw(), midi_driver_callback_wrapper::<T>, user_data);

            return MidiDriver {
                c_fluid_midi_driver: router
            }
        }

        fn midi_driver_callback_wrapper<T>(closure: *mut c_void, event: *mut fluid_midi_event_t) -> i32
            where T: Fn(MidiEvent) -> i32 {
            let closure = closure as *mut T;

            unsafe {
                (*closure)(MidiEvent::from_raw(event))
            }
        }
    }

    pub fn to_raw(&self) -> *mut fluid_midi_driver_t {
        self.c_fluid_midi_driver
    }
}

impl Drop for MidiDriver {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_midi_driver(self.c_fluid_midi_driver);
        }
    }
}



pub struct MidiEvent {
    c_fluid_midi_event: *mut fluid_midi_event_t
}

impl MidiEvent {
    pub fn new() -> MidiEvent {
        unsafe {
            MidiEvent {
                c_fluid_midi_event: new_fluid_midi_event()
            }
        }
    }

    pub fn from_raw(event: *mut fluid_midi_event_t) -> MidiEvent {
        MidiEvent {
            c_fluid_midi_event: event 
        }
    }

    pub fn get_type(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_type(self.to_raw()) 
        }
    }

    pub fn to_raw(&self) -> *mut fluid_midi_event_t {
        self.c_fluid_midi_event
    }
}

impl Drop for MidiEvent {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_midi_event(self.c_fluid_midi_event);
        }
    }
}


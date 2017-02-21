extern crate libc;
use libc::{c_void, c_int, c_float};
use std::ffi::{CString, CStr};
use ffi::*;
use settings::*;
use event::*;
use synth::Synth;
use std::mem::*;


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

    pub fn set_default_rules(&self) -> bool {
        unsafe {
            fluid_midi_router_set_default_rules(self.to_raw()) == 0
        }
    }

    pub fn clear_rules(&self) -> i32 {
        unsafe { 
            fluid_midi_router_clear_rules(self.to_raw()) 
        }
    }

    pub fn add_rule(&self, rule: &mut MidiRouterRule, rule_type: MidiRouterRuleType) -> i32 {
        unsafe { 
            fluid_midi_router_add_rule(self.to_raw(), rule.to_raw(), rule_type as c_int) 
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

    pub fn handle_midi_event(&self, event: &MidiEvent) -> bool {
        unsafe {
            fluid_midi_router_handle_midi_event(self.to_raw() as *mut c_void, event.to_raw()) == 0
        }
    }

    pub fn dump_prerouter(&self, event: &MidiEvent) -> bool {
        unsafe {
            fluid_midi_dump_prerouter(self.to_raw() as *mut c_void, event.to_raw()) == 0
        }
    }

    pub fn dump_postrouter(&self, event: &MidiEvent) -> bool {
        unsafe {
            fluid_midi_dump_postrouter(self.to_raw() as *mut c_void, event.to_raw()) == 0
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

    pub fn set_type(&self, event_type: i32) {
        unsafe { 
            fluid_midi_event_set_type(self.to_raw(), event_type as c_int);
        }
    }

    pub fn get_type(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_type(self.to_raw()) 
        }
    }

    pub fn set_channel(&self, channel: i32) {
        unsafe { 
            fluid_midi_event_set_channel(self.to_raw(), channel as c_int);
        }
    }

    pub fn get_channel(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_channel(self.to_raw()) 
        }
    }

    pub fn set_key(&self, key: i32) {
        unsafe { 
            fluid_midi_event_set_key(self.to_raw(), key as c_int);
        }
    }

    pub fn get_key(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_key(self.to_raw()) 
        }
    }

    pub fn set_velocity(&self, velocity: i32) {
        unsafe { 
            fluid_midi_event_set_velocity(self.to_raw(), velocity as c_int);
        }
    }

    pub fn get_velocity(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_velocity(self.to_raw()) 
        }
    }

    pub fn set_control(&self, control: i32) {
        unsafe { 
            fluid_midi_event_set_control(self.to_raw(), control as c_int);
        }
    }

    pub fn get_control(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_control(self.to_raw()) 
        }
    }

    pub fn set_value(&self, value: i32) {
        unsafe { 
            fluid_midi_event_set_value(self.to_raw(), value as c_int);
        }
    }

    pub fn get_value(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_value(self.to_raw()) 
        }
    }

    pub fn set_program(&self, program: i32) {
        unsafe { 
            fluid_midi_event_set_program(self.to_raw(), program as c_int);
        }
    }

    pub fn get_program(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_program(self.to_raw()) 
        }
    }

    pub fn set_pitch(&self, pitch: i32) {
        unsafe { 
            fluid_midi_event_set_pitch(self.to_raw(), pitch as c_int);
        }
    }

    pub fn get_pitch(&self) -> i32 {
        unsafe { 
            fluid_midi_event_get_pitch(self.to_raw()) 
        }
    }

    // TODO
    // set_sysex() {
    // }

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

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum PlayerStatus {
    Ready = 0,
    Playing,
    Done,
}


pub struct Player {
    c_fluid_player: *mut fluid_player_t
}

impl Player {
    pub fn new(synth: &Synth) -> Player {
        unsafe {
            Player {
                c_fluid_player: new_fluid_player(synth.to_raw())
            }
        }
    }

    pub fn add(&self, file: &str) {
        let file_str = CString::new(file).unwrap();
        unsafe {
            fluid_player_add(self.to_raw(), file_str.as_ptr());
        }
    }

    pub fn play(&self) {
        unsafe {
            fluid_player_play(self.to_raw());
        }
    } 

    pub fn stop(&self) {
        unsafe {
            fluid_player_stop(self.to_raw());
        }
    } 

    pub fn join(&self) -> bool {
        unsafe {
            fluid_player_join(self.to_raw()) == 0
        }
    } 

    pub fn set_midi_tempo(&self, tempo: i32) {
        unsafe {
            fluid_player_set_midi_tempo(self.to_raw(), tempo as c_int);
        }
    } 

    pub fn set_bpm(&self, bpm: i32) {
        unsafe {
            fluid_player_set_bpm(self.to_raw(), bpm as c_int);
        }
    } 

    pub fn get_status(&self) -> PlayerStatus {
        unsafe {
            transmute(fluid_player_get_status(self.to_raw()))
        }
    }

    pub fn to_raw(&self) -> *mut fluid_player_t {
        self.c_fluid_player
    }
}

impl Drop for Player {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_player(self.c_fluid_player);
        }
    }
}


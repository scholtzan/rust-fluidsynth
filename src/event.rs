extern crate libc;
use ffi::*;
use libc::{c_int, c_uint, c_short};

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum SeqEventType {
    Note = 0,
    NoteOn,
    NoteOff,
    AllSoundsOff,
    AllNotesOff,
    BankSelect,
    ProgramChange,
    ProgramSelect,
    PitchBend,
    PitchWheelSens,
    Modulation,
    Sustain,
    ControlChange,
    Pan,
    Volume,
    ReverbSend,
    ChorusSend,
    Timer,
    AnyControlChange,
    ChannelPressure,
    SystemReset,
    Unregistering,
    LastEvent
}



pub struct Event {
    c_fluid_event: *mut fluid_event_t
}

impl Event {
    pub fn new() -> Event {
        unsafe {
            Event {
                c_fluid_event: new_fluid_event()
            }
        }
    }

    pub fn from_raw(raw_event: *mut fluid_event_t) -> Event {
        Event {
             c_fluid_event: raw_event
        }
    }

    pub fn set_source(&self, src: i16) {
        unsafe {
            fluid_event_set_source(self.to_raw(), src as c_short);
        }
    }

    pub fn set_destination(&self, dest: i16) {
        unsafe {
            fluid_event_set_dest(self.to_raw(), dest as c_short);
        }
    }

    // TODO
    /*pub fn timer(&self, ) {
    
    }*/

    // TODO
    /*pub fn note(&self) {
    
    }*/

    pub fn noteon(&self, channel: i32, key: i32, velocity: i32) {
        unsafe {
            fluid_event_noteon(self.to_raw(), channel as c_int, key as c_short, velocity as c_short);
        }
    }

    pub fn noteoff(&self, channel: i32, key: i32) {
        unsafe {
            fluid_event_noteoff(self.to_raw(), channel as c_int, key as c_short);
        }
    }

    pub fn all_sounds_off(&self, channel: i32) {
        unsafe {
            fluid_event_all_sounds_off(self.to_raw(), channel as c_int);
        }
    }

    pub fn all_notes_off(&self, channel: i32) {
        unsafe {
            fluid_event_all_notes_off(self.to_raw(), channel as c_int);
        }
    }

    pub fn bank_select(&self, channel: i32, bank_num: i16) {
        unsafe {
            fluid_event_bank_select(self.to_raw(), channel as c_int, bank_num as c_short);
        }
    }

    pub fn program_change(&self, channel: i32, prog_num: i16) {
        unsafe {
            fluid_event_program_change(self.to_raw(), channel as c_int, prog_num as c_short);
        }
    }

    pub fn program_select(&self, channel: i32, sfont_id: u32, bank_num: i16, preset_num: i16) {
        unsafe {
            fluid_event_program_select(self.to_raw(), channel as c_int, sfont_id as c_uint, bank_num as c_short, preset_num as c_short);
        }
    }

    pub fn control_change(&self, channel: i32, control: i16, val: i16) {
        unsafe {
            fluid_event_control_change(self.to_raw(), channel as c_int, control as c_short, val as c_short);
        }
    }

    pub fn pitch_bend(&self, channel: i32, pitch: i32) {
        unsafe {
            fluid_event_pitch_bend(self.to_raw(), channel as c_int, pitch as c_int);
        }
    }

    pub fn pitch_wheelsens(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_pitch_wheelsens(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn pitch_modulation(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_modulation(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn sustain(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_sustain(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn pan(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_pan(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn volume(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_volume(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn reverb_send(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_reverb_send(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn chorus_send(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_chorus_send(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn channel_pressure(&self, channel: i32, value: i16) {
        unsafe {
            fluid_event_channel_pressure(self.to_raw(), channel as c_int, value as c_short);
        }
    }

    pub fn system_reset(&self) {
        unsafe {
            fluid_event_system_reset(self.to_raw());
        }
    }

    pub fn any_control_change(&self, channel: i32) {
        unsafe {
            fluid_event_any_control_change(self.to_raw(), channel as c_int);
        }
    }

    pub fn unregistering(&self) {
        unsafe {
            fluid_event_unregistering(self.to_raw());
        }
    }

    pub fn get_type(&self) -> i32 {
        unsafe {
            fluid_event_get_type(self.to_raw())
        }
    }

    pub fn get_source(&self) -> i16 {
        unsafe {
            fluid_event_get_source(self.to_raw())
        }
    }

    pub fn get_destination(&self) -> i16 {
        unsafe {
            fluid_event_get_dest(self.to_raw())
        }
    }

    pub fn get_channel(&self) -> i32 {
        unsafe {
            fluid_event_get_channel(self.to_raw())
        }
    }

    pub fn get_key(&self) -> i16 {
        unsafe {
            fluid_event_get_key(self.to_raw())
        }
    }

    pub fn get_velocity(&self) -> i16 {
        unsafe {
            fluid_event_get_velocity(self.to_raw())
        }
    }

    pub fn get_control(&self) -> i16 {
        unsafe {
            fluid_event_get_control(self.to_raw())
        }
    }

    pub fn get_value(&self) -> i16 {
        unsafe {
            fluid_event_get_value(self.to_raw())
        }
    }

    pub fn get_program(&self) -> i16 {
        unsafe {
            fluid_event_get_program(self.to_raw())
        }
    }

    // TODO
    /*pub fn get_event_data(&self) {
        unsafe {
            fluid_event_get_data(self.to_raw())
        }
    }*/

    pub fn get_duration(&self) -> u32 {
        unsafe {
            fluid_event_get_duration(self.to_raw())
        }
    }

    pub fn get_bank(&self) -> i16 {
        unsafe {
            fluid_event_get_bank(self.to_raw())
        }
    }

    pub fn get_pitch(&self) -> i32 {
        unsafe {
            fluid_event_get_pitch(self.to_raw())
        }
    }

    pub fn get_sfont_id(&self) -> u32 {
        unsafe {
            fluid_event_get_sfont_id(self.to_raw())
        }
    }


    pub fn to_raw(&self) -> *mut fluid_event_t {
        self.c_fluid_event
    }
}

impl Drop for Event {
    fn drop(&mut self) -> () {
        unsafe {
            delete_fluid_event(self.to_raw()); 
        }
    }
}


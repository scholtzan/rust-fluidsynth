extern crate libc;
use libc::{c_int, c_uint, c_double, c_float};
use std::ffi::{CString, CStr};
use ffi::*;
use settings::*;
use sfont::*;
use std::str;
use std::mem::*;
use midi::MidiRouter;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum ChorusMod {
    Sine = 0,
    Triangle
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum Interpolation {
    None = 0,
    Linear = 1,
    fourthOrder = 4,
    seventhOrder = 7
}

pub struct ChannelInfo {
    pub assigned: i32,
    pub sfont_id: i32,
    pub bank: i32,
    pub program: i32,
    pub name: String,
    _c_fluid_channel_info: *mut fluid_synth_channel_info_t
}

impl ChannelInfo {
    pub fn from_raw(raw_channel_info: *mut fluid_synth_channel_info_t) -> ChannelInfo {
        unsafe {
            ChannelInfo {
                assigned: (*raw_channel_info).assigned,
                sfont_id: (*raw_channel_info).sfont_id,
                bank: (*raw_channel_info).bank,
                program: (*raw_channel_info).program,
                name: String::from_utf8(((*raw_channel_info).name).iter().map(|&x| x as u8).collect::<Vec<u8>>()).unwrap(),
                _c_fluid_channel_info: raw_channel_info
            }
        }
    }
}

pub struct Preset {
    c_fluid_preset: *mut fluid_preset_t
}

impl Preset {
    pub fn from_raw(preset: *mut fluid_preset_t) -> Preset {
        Preset {
            c_fluid_preset: preset
        }
    }

    pub fn to_raw(&self) -> *mut fluid_preset_t {
        self.c_fluid_preset
    } 
}

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

    pub fn get_settings(&self) -> Settings {
        unsafe {
            Settings::from_raw(fluid_synth_get_settings(self.c_fluid_synth))
        }                          
    }

    pub fn noteon(&self, chan: i32, key: i32, vel: i32) -> i32 {
        unsafe {
            fluid_synth_noteon(self.c_fluid_synth, chan as c_int, key as c_int, vel as c_int)
        }                          
    }

    pub fn noteoff(&self, chan: i32, key: i32) -> i32 {
        unsafe {
            fluid_synth_noteoff(self.c_fluid_synth, chan as c_int, key as c_int)
        }                          
    }

    pub fn cc(&self, chan: i32, num: i32, val: i32) -> bool {
        unsafe {
            fluid_synth_cc(self.c_fluid_synth, chan as c_int, num as c_int, val as c_int) == 0
        }                          
    }

    pub fn get_cc(&self, chan: i32, num: i32) -> Option<i32> {
        unsafe {
            let pval: i32 = 0;
            let res = fluid_synth_get_cc(self.c_fluid_synth, chan as c_int, num as c_int, pval as *mut c_int);
            if res == 0 {
                None
            } else {
                Some(pval)
            }
        }                          
    }

    // TODO
    /* pub fn sysex() {
     * }
     */

    pub fn pitch_bend(&self, chan: i32, val: i32) -> bool {
        unsafe {
            fluid_synth_pitch_bend(self.c_fluid_synth, chan as c_int, val as c_int) == 0
        }                          
    }

    pub fn get_pitch_bend(&self, chan: i32) -> Option<i32> {
        unsafe {
            let ppitch: i32 = 0;

            let res = fluid_synth_get_pitch_bend(self.c_fluid_synth, chan as c_int, ppitch as *mut c_int);

            if res == 0 {
                None
            } else {
                Some(ppitch)
            }
        }                          
    }

    pub fn pitch_wheel_sens(&self, chan: i32, val: i32) -> bool {
        unsafe {
            fluid_synth_pitch_wheel_sens(self.c_fluid_synth, chan as c_int, val as c_int) == 0
        }                          
    }

    pub fn get_pitch_wheel_sens(&self, chan: i32) -> Option<i32> {
        unsafe {
            let ppitch: i32 = 0;

            let res = fluid_synth_get_pitch_wheel_sens(self.c_fluid_synth, chan as c_int, ppitch as *mut c_int);

            if res == 0 {
                None
            } else {
                Some(ppitch)
            }
        }                          
    }

    pub fn program_change(&self, chan: i32, prognum: i32) -> bool {
        unsafe {
            fluid_synth_program_change(self.c_fluid_synth, chan as c_int, prognum as c_int) == 0
        }                          
    }

    pub fn channel_pressure(&self, chan: i32, val: i32) -> bool {
        unsafe {
            fluid_synth_channel_pressure(self.c_fluid_synth, chan as c_int, val as c_int) == 0
        }                          
    }

    pub fn bank_select(&self, chan: i32, bank: u32) -> bool {
        unsafe {
            fluid_synth_bank_select(self.c_fluid_synth, chan as c_int, bank as c_uint) == 0
        }                          
    }

    pub fn sfont_select(&self, chan: i32, sfont_id: u32) -> bool {
        unsafe {
            fluid_synth_sfont_select(self.c_fluid_synth, chan as c_int, sfont_id as c_uint) == 0
        }                          
    }

    pub fn program_select(&self, chan: i32, sfont_id: u32, bank_num: u32, preset_num: u32) -> bool {
        unsafe {
            fluid_synth_program_select(self.c_fluid_synth, chan as c_int, sfont_id as c_uint, bank_num as c_uint, preset_num as c_uint) == 0
        }                          
    }

    pub fn program_select_by_sfont_name(&self, chan: i32, sfont_name: &str, bank_num: u32, preset_num: u32) -> bool {
        unsafe {
            let sfont_name_str = CString::new(sfont_name).unwrap().as_ptr();
            fluid_synth_program_select_by_sfont_name(self.c_fluid_synth, chan as c_int, sfont_name_str, bank_num as c_uint, preset_num as c_uint) == 0
        }                          
    }

    /* TODO
     * pub fn get_program() {
     * }
     */

    pub fn unset_program(&self, chan: i32) -> bool {
        unsafe {
            fluid_synth_unset_program(self.c_fluid_synth, chan as c_int) == 0
        }                          
    }

    pub fn get_channel_info(&self, chan: i32) -> Option<ChannelInfo> {
        unsafe {
            let info: *mut fluid_synth_channel_info_t = uninitialized();
            let res = fluid_synth_get_channel_info(self.c_fluid_synth, chan as c_int, info);
            
            if res == 0 {
                None
            } else {
                Some(ChannelInfo::from_raw(info))
            }
        }
    }
     

    pub fn program_reset(&self) -> bool {
        unsafe {
            fluid_synth_program_reset(self.c_fluid_synth) == 0
        }                          
    }

    pub fn system_reset(&self) -> bool {
        unsafe {
            fluid_synth_system_reset(self.c_fluid_synth) == 0
        }                          
    }

    pub fn get_channel_preset(&self, channel: i32) -> Option<Preset> {
        unsafe {
            let res = fluid_synth_get_channel_preset(self.c_fluid_synth, channel as c_int);

            if res.is_null() {
                None
            } else {
                Some(Preset::from_raw(res))
            }
        }
    }

    pub fn start(&self, id: u32, preset: &Preset, audio_channel: i32, channel: i32, key: i32, vel: i32) -> bool {
        unsafe {
            fluid_synth_start(self.to_raw(), id as c_uint, preset.to_raw(), audio_channel as c_int, channel as c_int, key as c_int, vel as c_int) == 0
        }
    }
    
    pub fn stop(&self, id: u32) -> bool {
        unsafe {
            fluid_synth_stop(self.to_raw(), id as c_uint) == 0
        }
    }

    pub fn sfload(&self, filename: &str, reset_presets: i32) -> bool {
        unsafe {
            let filename_str = CString::new(filename).unwrap().as_ptr();
            fluid_synth_sfload(self.c_fluid_synth, filename_str, reset_presets as c_int) == 0
        }                    
    }

    pub fn sfreload(&self, id: u32) -> bool {
        unsafe {
            fluid_synth_sfreload(self.c_fluid_synth, id as c_uint) == 0
        }                    
    }

    pub fn sfunload(&self, id: u32, reset_presets: bool) -> bool {
        unsafe {
            fluid_synth_sfunload(self.c_fluid_synth, id as c_uint, reset_presets as c_int) == 0
        }                    
    }

    pub fn add_sfont(&self, sfont: &SoundFont) -> Option<i32> {
        unsafe {
            let result = fluid_synth_add_sfont(self.c_fluid_synth, sfont.to_raw());

            if result == -1 {
                None
            } else {
                Some(result)
            }
        }                    
    }

    pub fn remove_sfont(&self, sfont: &SoundFont) {
        unsafe {
            fluid_synth_remove_sfont(self.c_fluid_synth, sfont.to_raw());
        }                    
    }

    pub fn sfcount(&self) -> i32 {
        unsafe {
            fluid_synth_sfcount(self.c_fluid_synth)
        }                    
    }

    pub fn get_sfont(&self, num: u32) -> Option<SoundFont> {
        unsafe {
            let result = fluid_synth_get_sfont(self.to_raw(), num as c_uint);

            if result.is_null() {
                None
            } else {
                Some(SoundFont::from_raw(result))
            }
        }
    }

    pub fn get_sfont_by_id(&self, id: u32) -> Option<SoundFont> {
        unsafe {
            let result = fluid_synth_get_sfont_by_id(self.to_raw(), id as c_uint);

            if result.is_null() {
                None
            } else {
                Some(SoundFont::from_raw(result))
            }
        }
    }

    pub fn get_sfont_by_name(&self, name: &str) -> Option<SoundFont> {
        unsafe {
            let name_str = CString::new(name).unwrap().as_ptr();
            let result = fluid_synth_get_sfont_by_name(self.to_raw(), name_str);

            if result.is_null() {
                None
            } else {
                Some(SoundFont::from_raw(result))
            }
        }
    }

    pub fn set_bank_offset(&self, sfont_id: i32, offset: i32) -> bool {
        unsafe {
            fluid_synth_set_bank_offset(self.c_fluid_synth, sfont_id as c_int, offset as c_int) == 0
        }                    
    }

    pub fn get_bank_offset(&self, sfont_id: i32) -> i32 {
        unsafe {
            fluid_synth_get_bank_offset(self.c_fluid_synth, sfont_id as c_int)
        }                    
    }

    pub fn set_reverb(&self, roomsize: f64, dumping: f64, width: f64, level: f64) {
        unsafe {
            fluid_synth_set_reverb(self.to_raw(), roomsize as c_double, dumping as c_double, width as c_double, level as c_double);
        }
    }

    pub fn set_reverb_on(&self, on: bool) {
        unsafe {
            fluid_synth_set_reverb_on(self.to_raw(), on as c_int);
        }
    }

    pub fn get_reverb_roomsize(&self) -> f64 {
        unsafe {
            fluid_synth_get_reverb_roomsize(self.to_raw())
        }
    }

    pub fn get_reverb_damp(&self) -> f64 {
        unsafe {
            fluid_synth_get_reverb_damp(self.to_raw())
        }
    }

    pub fn get_reverb_level(&self) -> f64 {
        unsafe {
            fluid_synth_get_reverb_level(self.to_raw())
        }
    }

    pub fn set_chorus(&self, nr: i32, level: f64, speed: f64, depth_ms: f64, chorus_mod: ChorusMod) {
        unsafe {
            fluid_synth_set_chorus(self.to_raw(), nr as c_int, level as c_double, speed as c_double, depth_ms as c_double, chorus_mod as c_int)
        }
    }

    pub fn set_chorus_on(&self, on: bool) {
        unsafe {
            fluid_synth_set_chorus_on(self.to_raw(), on as c_int);
        }
    }

    pub fn get_chorus_nr(&self) -> i32 {
        unsafe {
            fluid_synth_get_chorus_nr(self.to_raw())
        }
    }

    pub fn get_chorus_level(&self) -> f64 {
        unsafe {
            fluid_synth_get_chorus_level(self.to_raw())
        }
    }

    pub fn get_chorus_speed_hz(&self) -> f64 {
        unsafe {
            fluid_synth_get_chorus_speed_Hz(self.to_raw())
        }
    }

    pub fn get_chorus_depth_ms(&self) -> f64 {
        unsafe {
            fluid_synth_get_chorus_depth_ms(self.to_raw())
        }
    }

    pub fn get_chorus_type(&self) -> ChorusMod {
        unsafe {
            transmute(fluid_synth_get_chorus_type(self.to_raw()))
        }
    }

    pub fn count_midi_channels(&self) -> i32 {
        unsafe {
            fluid_synth_count_midi_channels(self.to_raw())
        }
    }

    pub fn count_audio_channels(&self) -> i32 {
        unsafe {
            fluid_synth_count_audio_channels(self.to_raw())
        }
    }

    pub fn count_audio_groups(&self) -> i32 {
        unsafe {
            fluid_synth_count_audio_groups(self.to_raw())
        }
    }

    pub fn count_effects_channels(&self) -> i32 {
        unsafe {
            fluid_synth_count_effects_channels(self.to_raw())
        }
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        unsafe {
            fluid_synth_set_sample_rate(self.to_raw(), sample_rate as c_float)
        }
    }

    pub fn set_gain(&self, gain: f32) {
        unsafe {
            fluid_synth_set_gain(self.to_raw(), gain as c_float)
        }
    }

    pub fn get_gain(&self) -> f32 {
        unsafe {
            fluid_synth_get_gain(self.to_raw())
        }
    }

    pub fn set_polyphony(&self, polyphony: i32) -> bool {
        unsafe {
            fluid_synth_set_polyphony(self.to_raw(), polyphony as c_int) == 0
        }
    }

    pub fn get_polyphony(&self) -> i32 {
        unsafe {
            fluid_synth_get_polyphony(self.to_raw())
        }
    }

    pub fn get_active_voice_count(&self) -> i32 {
        unsafe {
            fluid_synth_get_active_voice_count(self.to_raw())
        }
    }

    pub fn get_internal_bufsize(&self) -> i32 {
        unsafe {
            fluid_synth_get_internal_bufsize(self.to_raw())
        }
    }

    pub fn set_interp_method(&self, chan: i32, interp_method: i32) -> bool {
        unsafe {
            fluid_synth_set_interp_method(self.to_raw(), chan as c_int, interp_method as c_int) == 0
        }
    }

    pub fn set_gen(&self, chan: i32, param: i32, value: f32) -> bool {
        unsafe {
            fluid_synth_set_gen(self.to_raw(), chan as c_int, param as c_int, value as c_float) == 0
        }
    }

    pub fn set_gen2(&self, chan: i32, param: i32, value: f32, absolute: bool, normalized: bool) -> bool {
        unsafe {
            fluid_synth_set_gen2(self.to_raw(), chan as c_int, param as c_int, value as c_float, absolute as c_int, normalized as c_int) == 0
        }
    }

    pub fn get_gen(&self, chan: i32, param: i32) -> f32 {
        unsafe {
            fluid_synth_get_gen(self.to_raw(), chan as c_int, param as c_int)
        }
    }

    pub fn create_key_tuning(&self, bank: i32, prog: i32, name: &str, pitch: *const f64) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap().as_ptr();
            fluid_synth_create_key_tuning(self.to_raw(), bank as c_int, prog as c_int, name_str, pitch as *const c_double) == 0
        }
    }

    pub fn activate_key_tuning(&self, bank: i32, prog: i32, name: &str, pitch: *const f64, apply: bool) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap().as_ptr();
            fluid_synth_activate_key_tuning(self.to_raw(), bank as c_int, prog as c_int, name_str, pitch as *const c_double, apply as c_int) == 0
        }
    }

    pub fn create_octave_tuning(&self, bank: i32, prog: i32, name: &str, pitch: *const f64) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap().as_ptr();
            fluid_synth_create_octave_tuning(self.to_raw(), bank as c_int, prog as c_int, name_str, pitch as *const c_double) == 0
        }
    }

    pub fn activate_octave_tuning(&self, bank: i32, prog: i32, name: &str, pitch: *const f64, apply: bool) -> bool {
        unsafe {
            let name_str = CString::new(name).unwrap().as_ptr();
            fluid_synth_activate_octave_tuning(self.to_raw(), bank as c_int, prog as c_int, name_str, pitch as *const c_double, apply as c_int) == 0
        }
    }

    pub fn tune_notes(&self, bank: i32, prog: i32, len: i32, key: *const i32, pitch: *const f64, apply: bool) -> bool {
        unsafe {
            fluid_synth_tune_notes(self.to_raw(), bank as c_int, prog as c_int, len as c_int, key as *const c_int, pitch as *const c_double, apply as c_int) == 0
        }
    }

    pub fn select_tuning(&self, chan: i32, bank: i32, prog: i32) -> bool {
        unsafe {
            fluid_synth_select_tuning(self.to_raw(), chan as c_int, bank as c_int, prog as c_int) == 0
        }
    }

    pub fn activate_tuning(&self, chan: i32, bank: i32, prog: i32, apply: bool) -> bool {
        unsafe {
            fluid_synth_activate_tuning(self.to_raw(), chan as c_int, bank as c_int, prog as c_int, apply as c_int) == 0
        }
    }

    pub fn reset_tuning(&self, chan: i32) -> bool {
        unsafe {
            fluid_synth_reset_tuning(self.to_raw(), chan as c_int) == 0
        }
    }

    pub fn deactivate_tuning(&self, chan: i32, apply: bool) -> bool {
        unsafe {
            fluid_synth_deactivate_tuning(self.to_raw(), chan as c_int, apply as c_int) == 0
        }
    }

    pub fn tuning_iteration_start(&self) {
        unsafe {
            fluid_synth_tuning_iteration_start(self.to_raw())
        }
    }

    // TODO
    // tuning_iteration_next() {
    // }
    //
    // tuning_dumping() {
    // }

    pub fn get_cpu_load(&self) -> f64 {
        unsafe {
            fluid_synth_get_cpu_load(self.to_raw())
        }
    }

    pub fn error(&self) -> &str {
        unsafe {
            str::from_utf8(CStr::from_ptr(fluid_synth_error(self.to_raw())).to_bytes()).unwrap()
        }
    }

    // TODO
    // [...]

    pub fn set_midi_router(&self, router: &MidiRouter) {
        unsafe {
            fluid_synth_set_midi_router(self.to_raw(), router.to_raw())
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


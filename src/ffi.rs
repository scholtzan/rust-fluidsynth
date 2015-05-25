#![allow(non_camel_case_types)]

pub enum Struct__fluid_hashtable_t { }
pub type fluid_settings_t = Struct__fluid_hashtable_t;
pub enum Struct__fluid_synth_t {}
pub type fluid_synth_t = Struct__fluid_synth_t;
pub type fluid_synth_channel_info_t = Struct__fluid_synth_channel_info_t;
pub enum Struct__fluid_voice_t { }
pub type fluid_voice_t = Struct__fluid_voice_t;
pub type fluid_sfloader_t = Struct__fluid_sfloader_t;
pub type fluid_sfont_t = Struct__fluid_sfont_t;
pub type fluid_preset_t = Struct__fluid_preset_t;
pub type fluid_sample_t = Struct__fluid_sample_t;
pub type fluid_mod_t = Struct__fluid_mod_t;
pub enum Struct__fluid_audio_driver_t { }
pub type fluid_audio_driver_t = Struct__fluid_audio_driver_t;
pub enum Struct__fluid_file_renderer_t { }
pub type fluid_file_renderer_t = Struct__fluid_file_renderer_t;
pub enum Struct__fluid_player_t { }
pub type fluid_player_t = Struct__fluid_player_t;
pub enum Struct__fluid_midi_event_t { }
pub type fluid_midi_event_t = Struct__fluid_midi_event_t;
pub enum Struct__fluid_midi_driver_t { }
pub type fluid_midi_driver_t = Struct__fluid_midi_driver_t;
pub enum Struct__fluid_midi_router_t { }
pub type fluid_midi_router_t = Struct__fluid_midi_router_t;
pub enum Struct__fluid_midi_router_rule_t { }
pub type fluid_midi_router_rule_t = Struct__fluid_midi_router_rule_t;
pub type fluid_cmd_handler_t = Struct__fluid_hashtable_t;
pub enum Struct__fluid_shell_t { }
pub type fluid_shell_t = Struct__fluid_shell_t;
pub enum Struct__fluid_server_t { }
pub type fluid_server_t = Struct__fluid_server_t;
pub enum Struct__fluid_event_t { }
pub type fluid_event_t = Struct__fluid_event_t;
pub enum Struct__fluid_sequencer_t { }
pub type fluid_sequencer_t = Struct__fluid_sequencer_t;
pub enum Struct__fluid_ramsfont_t { }
pub type fluid_ramsfont_t = Struct__fluid_ramsfont_t;
pub enum Struct__fluid_rampreset_t { }
pub type fluid_rampreset_t = Struct__fluid_rampreset_t;
pub type fluid_istream_t = ::libc::c_int;
pub type fluid_ostream_t = ::libc::c_int;

#[repr(C)]
pub struct Struct__fluid_synth_channel_info_t {
    pub assigned: ::libc::c_int,
    pub sfont_id: ::libc::c_int,
    pub bank: ::libc::c_int,
    pub program: ::libc::c_int,
    pub name: [::libc::c_char; 32usize],
    pub reserved: [::libc::c_char; 32usize],
}

pub type Enum_fluid_midi_channel_type = ::libc::c_uint;
pub const CHANNEL_TYPE_MELODIC: ::libc::c_uint = 0;
pub const CHANNEL_TYPE_DRUM: ::libc::c_uint = 1;


pub type fluid_audio_callback_t =
    ::std::option::Option<extern "C" fn
                              (synth: *mut fluid_synth_t, len: ::libc::c_int,
                               out1: *mut ::libc::c_void, loff: ::libc::c_int,
                               lincr: ::libc::c_int,
                               out2: *mut ::libc::c_void, roff: ::libc::c_int,
                               rincr: ::libc::c_int) -> ::libc::c_int>;
pub type fluid_cmd_func_t =
    ::std::option::Option<extern "C" fn
                              (data: *mut ::libc::c_void, ac: ::libc::c_int,
                               av: *mut *mut ::libc::c_char,
                               out: fluid_ostream_t) -> ::libc::c_int>;
#[repr(C)]
pub struct Struct_Unnamed2 {
    pub name: *mut ::libc::c_char,
    pub topic: *mut ::libc::c_char,
    pub handler: fluid_cmd_func_t,
    pub data: *mut ::libc::c_void,
    pub help: *mut ::libc::c_char,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Struct_Unnamed2 { unsafe { ::std::mem::zeroed() } }
}
pub type fluid_cmd_t = Struct_Unnamed2;
pub type fluid_server_newclient_func_t =
    ::std::option::Option<extern "C" fn
                              (data: *mut ::libc::c_void,
                               addr: *mut ::libc::c_char)
                              -> *mut fluid_cmd_handler_t>;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const FLUID_PRESET_SELECTED: ::libc::c_uint = 0;
pub const FLUID_PRESET_UNSELECTED: ::libc::c_uint = 1;
pub const FLUID_SAMPLE_DONE: ::libc::c_uint = 2;

#[repr(C)]
pub struct Struct__fluid_sfloader_t {
    pub data: *mut ::libc::c_void,
    pub free: ::std::option::Option<extern "C" fn
                                        (loader: *mut fluid_sfloader_t)
                                        -> ::libc::c_int>,
    pub load: ::std::option::Option<extern "C" fn
                                        (loader: *mut fluid_sfloader_t,
                                         filename: *const ::libc::c_char)
                                        -> *mut fluid_sfont_t>,
}

#[repr(C)]
pub struct Struct__fluid_sfont_t {
    pub data: *mut ::libc::c_void,
    pub id: ::libc::c_uint,
    pub free: ::std::option::Option<extern "C" fn(sfont: *mut fluid_sfont_t)
                                        -> ::libc::c_int>,
    pub get_name: ::std::option::Option<extern "C" fn
                                            (sfont: *mut fluid_sfont_t)
                                            -> *mut ::libc::c_char>,
    pub get_preset: ::std::option::Option<extern "C" fn
                                              (sfont: *mut fluid_sfont_t,
                                               bank: ::libc::c_uint,
                                               prenum: ::libc::c_uint)
                                              -> *mut fluid_preset_t>,
    pub iteration_start: ::std::option::Option<extern "C" fn
                                                   (sfont: *mut fluid_sfont_t)
                                                   -> ()>,
    pub iteration_next: ::std::option::Option<extern "C" fn
                                                  (sfont: *mut fluid_sfont_t,
                                                   preset:
                                                       *mut fluid_preset_t)
                                                  -> ::libc::c_int>,
}

#[repr(C)]
pub struct Struct__fluid_preset_t {
    pub data: *mut ::libc::c_void,
    pub sfont: *mut fluid_sfont_t,
    pub free: ::std::option::Option<extern "C" fn(preset: *mut fluid_preset_t)
                                        -> ::libc::c_int>,
    pub get_name: ::std::option::Option<extern "C" fn
                                            (preset: *mut fluid_preset_t)
                                            -> *mut ::libc::c_char>,
    pub get_banknum: ::std::option::Option<extern "C" fn
                                               (preset: *mut fluid_preset_t)
                                               -> ::libc::c_int>,
    pub get_num: ::std::option::Option<extern "C" fn
                                           (preset: *mut fluid_preset_t)
                                           -> ::libc::c_int>,
    pub noteon: ::std::option::Option<extern "C" fn
                                          (preset: *mut fluid_preset_t,
                                           synth: *mut fluid_synth_t,
                                           chan: ::libc::c_int,
                                           key: ::libc::c_int,
                                           vel: ::libc::c_int)
                                          -> ::libc::c_int>,
    pub notify: ::std::option::Option<extern "C" fn
                                          (preset: *mut fluid_preset_t,
                                           reason: ::libc::c_int,
                                           chan: ::libc::c_int)
                                          -> ::libc::c_int>,
}

#[repr(C)]
pub struct Struct__fluid_sample_t {
    pub name: [::libc::c_char; 21usize],
    pub start: ::libc::c_uint,
    pub end: ::libc::c_uint,
    pub loopstart: ::libc::c_uint,
    pub loopend: ::libc::c_uint,
    pub samplerate: ::libc::c_uint,
    pub origpitch: ::libc::c_int,
    pub pitchadj: ::libc::c_int,
    pub sampletype: ::libc::c_int,
    pub valid: ::libc::c_int,
    pub data: *mut ::libc::c_short,
    pub amplitude_that_reaches_noise_floor_is_valid: ::libc::c_int,
    pub amplitude_that_reaches_noise_floor: ::libc::c_double,
    pub refcount: ::libc::c_uint,
    pub notify: ::std::option::Option<extern "C" fn
                                          (sample: *mut fluid_sample_t,
                                           reason: ::libc::c_int)
                                          -> ::libc::c_int>,
    pub userdata: *mut ::libc::c_void,
}

pub type fluid_audio_func_t =
    ::std::option::Option<extern "C" fn
                              (data: *mut ::libc::c_void, len: ::libc::c_int,
                               nin: ::libc::c_int,
                               _in: *mut *mut ::libc::c_float,
                               nout: ::libc::c_int,
                               out: *mut *mut ::libc::c_float)
                              -> ::libc::c_int>;

pub type handle_midi_event_func_t =
    ::std::option::Option<extern "C" fn
                              (data: *mut ::libc::c_void,
                               event: *mut fluid_midi_event_t)
                              -> ::libc::c_int>;
pub type Enum_fluid_player_status = ::libc::c_uint;
pub const FLUID_PLAYER_READY: ::libc::c_uint = 0;
pub const FLUID_PLAYER_PLAYING: ::libc::c_uint = 1;
pub const FLUID_PLAYER_DONE: ::libc::c_uint = 2;
pub type fluid_log_function_t =
    ::std::option::Option<extern "C" fn
                              (level: ::libc::c_int,
                               message: *mut ::libc::c_char,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
pub struct Struct__fluid_mod_t {
    pub dest: ::libc::c_uchar,
    pub src1: ::libc::c_uchar,
    pub flags1: ::libc::c_uchar,
    pub src2: ::libc::c_uchar,
    pub flags2: ::libc::c_uchar,
    pub amount: ::libc::c_double,
    pub next: *mut fluid_mod_t,
}

#[repr(C)]
pub struct Struct__fluid_gen_t {
    pub flags: ::libc::c_uchar,
    pub val: ::libc::c_double,
    pub _mod: ::libc::c_double,
    pub nrpn: ::libc::c_double,
}
pub type fluid_gen_t = Struct__fluid_gen_t;

#[link(name = "fluidsynth")]
extern "C" {
    pub fn new_fluid_settings() -> *mut fluid_settings_t;
    pub fn delete_fluid_settings(settings: *mut fluid_settings_t) -> ();
    pub fn fluid_settings_get_type(settings: *mut fluid_settings_t,
                                   name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_get_hints(settings: *mut fluid_settings_t,
                                    name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_is_realtime(settings: *mut fluid_settings_t,
                                      name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_setstr(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 str: *const ::libc::c_char) -> ::libc::c_int;
    pub fn fluid_settings_copystr(settings: *mut fluid_settings_t,
                                  name: *const ::libc::c_char,
                                  str: *mut ::libc::c_char,
                                  len: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_settings_dupstr(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 str: *mut *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_getstr(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 str: *mut *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_getstr_default(settings: *mut fluid_settings_t,
                                         name: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn fluid_settings_str_equal(settings: *mut fluid_settings_t,
                                    name: *const ::libc::c_char,
                                    value: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_setnum(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 val: ::libc::c_double) -> ::libc::c_int;
    pub fn fluid_settings_getnum(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 val: *mut ::libc::c_double) -> ::libc::c_int;
    pub fn fluid_settings_getnum_default(settings: *mut fluid_settings_t,
                                         name: *const ::libc::c_char)
     -> ::libc::c_double;
    pub fn fluid_settings_getnum_range(settings: *mut fluid_settings_t,
                                       name: *const ::libc::c_char,
                                       min: *mut ::libc::c_double,
                                       max: *mut ::libc::c_double) -> ();
    pub fn fluid_settings_setint(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 val: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_settings_getint(settings: *mut fluid_settings_t,
                                 name: *const ::libc::c_char,
                                 val: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_settings_getint_default(settings: *mut fluid_settings_t,
                                         name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_getint_range(settings: *mut fluid_settings_t,
                                       name: *const ::libc::c_char,
                                       min: *mut ::libc::c_int,
                                       max: *mut ::libc::c_int) -> ();
    pub fn fluid_settings_foreach_option(settings: *mut fluid_settings_t,
                                         name: *const ::libc::c_char,
                                         data: *mut ::libc::c_void,
                                         func: extern fn (closure: *mut ::libc::c_void, name: *const ::libc::c_char, option:  *const ::libc::c_char))
     -> ();
    pub fn fluid_settings_option_count(settings: *mut fluid_settings_t,
                                       name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_settings_option_concat(settings: *mut fluid_settings_t,
                                        name: *const ::libc::c_char,
                                        separator: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn fluid_settings_foreach(settings: *mut fluid_settings_t,
                                  data: *mut ::libc::c_void,
                                  func: extern fn (closure: *mut ::libc::c_void, name: *const ::libc::c_char, setting_type: ::libc::c_int)) -> ();
    pub fn new_fluid_synth(settings: *mut fluid_settings_t)
     -> *mut fluid_synth_t;
    pub fn delete_fluid_synth(synth: *mut fluid_synth_t) -> ::libc::c_int;
    pub fn fluid_synth_get_settings(synth: *mut fluid_synth_t)
     -> *mut fluid_settings_t;
    pub fn fluid_synth_noteon(synth: *mut fluid_synth_t, chan: ::libc::c_int,
                              key: ::libc::c_int, vel: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_noteoff(synth: *mut fluid_synth_t, chan: ::libc::c_int,
                               key: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_cc(synth: *mut fluid_synth_t, chan: ::libc::c_int,
                          ctrl: ::libc::c_int, val: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_get_cc(synth: *mut fluid_synth_t, chan: ::libc::c_int,
                              ctrl: ::libc::c_int, pval: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_sysex(synth: *mut fluid_synth_t,
                             data: *const ::libc::c_char, len: ::libc::c_int,
                             response: *mut ::libc::c_char,
                             response_len: *mut ::libc::c_int,
                             handled: *mut ::libc::c_int,
                             dryrun: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_pitch_bend(synth: *mut fluid_synth_t,
                                  chan: ::libc::c_int, val: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_get_pitch_bend(synth: *mut fluid_synth_t,
                                      chan: ::libc::c_int,
                                      ppitch_bend: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_pitch_wheel_sens(synth: *mut fluid_synth_t,
                                        chan: ::libc::c_int,
                                        val: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_get_pitch_wheel_sens(synth: *mut fluid_synth_t,
                                            chan: ::libc::c_int,
                                            pval: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_program_change(synth: *mut fluid_synth_t,
                                      chan: ::libc::c_int,
                                      program: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_channel_pressure(synth: *mut fluid_synth_t,
                                        chan: ::libc::c_int,
                                        val: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_bank_select(synth: *mut fluid_synth_t,
                                   chan: ::libc::c_int, bank: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_sfont_select(synth: *mut fluid_synth_t,
                                    chan: ::libc::c_int,
                                    sfont_id: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_program_select(synth: *mut fluid_synth_t,
                                      chan: ::libc::c_int,
                                      sfont_id: ::libc::c_uint,
                                      bank_num: ::libc::c_uint,
                                      preset_num: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_program_select_by_sfont_name(synth: *mut fluid_synth_t,
                                                    chan: ::libc::c_int,
                                                    sfont_name:
                                                        *const ::libc::c_char,
                                                    bank_num: ::libc::c_uint,
                                                    preset_num:
                                                        ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_get_program(synth: *mut fluid_synth_t,
                                   chan: ::libc::c_int,
                                   sfont_id: *mut ::libc::c_uint,
                                   bank_num: *mut ::libc::c_uint,
                                   preset_num: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_unset_program(synth: *mut fluid_synth_t,
                                     chan: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_get_channel_info(synth: *mut fluid_synth_t,
                                        chan: ::libc::c_int,
                                        info: *mut fluid_synth_channel_info_t)
     -> ::libc::c_int;
    pub fn fluid_synth_program_reset(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_system_reset(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_all_notes_off(synth: *mut fluid_synth_t,
                                     chan: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_all_sounds_off(synth: *mut fluid_synth_t,
                                      chan: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_set_channel_type(synth: *mut fluid_synth_t,
                                        chan: ::libc::c_int,
                                        _type: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_get_channel_preset(synth: *mut fluid_synth_t,
                                          chan: ::libc::c_int)
     -> *mut fluid_preset_t;
    pub fn fluid_synth_start(synth: *mut fluid_synth_t, id: ::libc::c_uint,
                             preset: *mut fluid_preset_t,
                             audio_chan: ::libc::c_int,
                             midi_chan: ::libc::c_int, key: ::libc::c_int,
                             vel: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_stop(synth: *mut fluid_synth_t, id: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_sfload(synth: *mut fluid_synth_t,
                              filename: *const ::libc::c_char,
                              reset_presets: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_sfreload(synth: *mut fluid_synth_t, id: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_synth_sfunload(synth: *mut fluid_synth_t, id: ::libc::c_uint,
                                reset_presets: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_add_sfont(synth: *mut fluid_synth_t,
                                 sfont: *mut fluid_sfont_t) -> ::libc::c_int;
    pub fn fluid_synth_remove_sfont(synth: *mut fluid_synth_t,
                                    sfont: *mut fluid_sfont_t) -> ();
    pub fn fluid_synth_sfcount(synth: *mut fluid_synth_t) -> ::libc::c_int;
    pub fn fluid_synth_get_sfont(synth: *mut fluid_synth_t,
                                 num: ::libc::c_uint) -> *mut fluid_sfont_t;
    pub fn fluid_synth_get_sfont_by_id(synth: *mut fluid_synth_t,
                                       id: ::libc::c_uint)
     -> *mut fluid_sfont_t;
    pub fn fluid_synth_get_sfont_by_name(synth: *mut fluid_synth_t,
                                         name: *const ::libc::c_char)
     -> *mut fluid_sfont_t;
    pub fn fluid_synth_set_bank_offset(synth: *mut fluid_synth_t,
                                       sfont_id: ::libc::c_int,
                                       offset: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_get_bank_offset(synth: *mut fluid_synth_t,
                                       sfont_id: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_set_reverb(synth: *mut fluid_synth_t,
                                  roomsize: ::libc::c_double,
                                  damping: ::libc::c_double,
                                  width: ::libc::c_double,
                                  level: ::libc::c_double) -> ();
    pub fn fluid_synth_set_reverb_on(synth: *mut fluid_synth_t,
                                     on: ::libc::c_int) -> ();
    pub fn fluid_synth_get_reverb_roomsize(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_get_reverb_damp(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_get_reverb_level(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_get_reverb_width(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_set_chorus(synth: *mut fluid_synth_t,
                                  nr: ::libc::c_int, level: ::libc::c_double,
                                  speed: ::libc::c_double,
                                  depth_ms: ::libc::c_double,
                                  _type: ::libc::c_int) -> ();
    pub fn fluid_synth_set_chorus_on(synth: *mut fluid_synth_t,
                                     on: ::libc::c_int) -> ();
    pub fn fluid_synth_get_chorus_nr(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_get_chorus_level(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_get_chorus_speed_Hz(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_get_chorus_depth_ms(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_get_chorus_type(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_count_midi_channels(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_count_audio_channels(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_count_audio_groups(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_count_effects_channels(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_set_sample_rate(synth: *mut fluid_synth_t,
                                       sample_rate: ::libc::c_float) -> ();
    pub fn fluid_synth_set_gain(synth: *mut fluid_synth_t,
                                gain: ::libc::c_float) -> ();
    pub fn fluid_synth_get_gain(synth: *mut fluid_synth_t) -> ::libc::c_float;
    pub fn fluid_synth_set_polyphony(synth: *mut fluid_synth_t,
                                     polyphony: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_get_polyphony(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_get_active_voice_count(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_get_internal_bufsize(synth: *mut fluid_synth_t)
     -> ::libc::c_int;
    pub fn fluid_synth_set_interp_method(synth: *mut fluid_synth_t,
                                         chan: ::libc::c_int,
                                         interp_method: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_set_gen(synth: *mut fluid_synth_t, chan: ::libc::c_int,
                               param: ::libc::c_int, value: ::libc::c_float)
     -> ::libc::c_int;
    pub fn fluid_synth_set_gen2(synth: *mut fluid_synth_t,
                                chan: ::libc::c_int, param: ::libc::c_int,
                                value: ::libc::c_float,
                                absolute: ::libc::c_int,
                                normalized: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_get_gen(synth: *mut fluid_synth_t, chan: ::libc::c_int,
                               param: ::libc::c_int) -> ::libc::c_float;
    pub fn fluid_synth_create_key_tuning(synth: *mut fluid_synth_t,
                                         bank: ::libc::c_int,
                                         prog: ::libc::c_int,
                                         name: *const ::libc::c_char,
                                         pitch: *const ::libc::c_double)
     -> ::libc::c_int;
    pub fn fluid_synth_activate_key_tuning(synth: *mut fluid_synth_t,
                                           bank: ::libc::c_int,
                                           prog: ::libc::c_int,
                                           name: *const ::libc::c_char,
                                           pitch: *const ::libc::c_double,
                                           apply: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_create_octave_tuning(synth: *mut fluid_synth_t,
                                            bank: ::libc::c_int,
                                            prog: ::libc::c_int,
                                            name: *const ::libc::c_char,
                                            pitch: *const ::libc::c_double)
     -> ::libc::c_int;
    pub fn fluid_synth_activate_octave_tuning(synth: *mut fluid_synth_t,
                                              bank: ::libc::c_int,
                                              prog: ::libc::c_int,
                                              name: *const ::libc::c_char,
                                              pitch: *const ::libc::c_double,
                                              apply: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_tune_notes(synth: *mut fluid_synth_t,
                                  bank: ::libc::c_int, prog: ::libc::c_int,
                                  len: ::libc::c_int,
                                  keys: *const ::libc::c_int,
                                  pitch: *const ::libc::c_double,
                                  apply: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_select_tuning(synth: *mut fluid_synth_t,
                                     chan: ::libc::c_int, bank: ::libc::c_int,
                                     prog: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_activate_tuning(synth: *mut fluid_synth_t,
                                       chan: ::libc::c_int,
                                       bank: ::libc::c_int,
                                       prog: ::libc::c_int,
                                       apply: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_reset_tuning(synth: *mut fluid_synth_t,
                                    chan: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_synth_deactivate_tuning(synth: *mut fluid_synth_t,
                                         chan: ::libc::c_int,
                                         apply: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_tuning_iteration_start(synth: *mut fluid_synth_t)
     -> ();
    pub fn fluid_synth_tuning_iteration_next(synth: *mut fluid_synth_t,
                                             bank: *mut ::libc::c_int,
                                             prog: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_tuning_dump(synth: *mut fluid_synth_t,
                                   bank: ::libc::c_int, prog: ::libc::c_int,
                                   name: *mut ::libc::c_char,
                                   len: ::libc::c_int,
                                   pitch: *mut ::libc::c_double)
     -> ::libc::c_int;
    pub fn fluid_synth_get_cpu_load(synth: *mut fluid_synth_t)
     -> ::libc::c_double;
    pub fn fluid_synth_error(synth: *mut fluid_synth_t)
     -> *mut ::libc::c_char;
    pub fn fluid_synth_write_s16(synth: *mut fluid_synth_t,
                                 len: ::libc::c_int,
                                 lout: *mut ::libc::c_void,
                                 loff: ::libc::c_int, lincr: ::libc::c_int,
                                 rout: *mut ::libc::c_void,
                                 roff: ::libc::c_int, rincr: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_write_float(synth: *mut fluid_synth_t,
                                   len: ::libc::c_int,
                                   lout: *mut ::libc::c_void,
                                   loff: ::libc::c_int, lincr: ::libc::c_int,
                                   rout: *mut ::libc::c_void,
                                   roff: ::libc::c_int, rincr: ::libc::c_int)
     -> ::libc::c_int;
    pub fn fluid_synth_nwrite_float(synth: *mut fluid_synth_t,
                                    len: ::libc::c_int,
                                    left: *mut *mut ::libc::c_float,
                                    right: *mut *mut ::libc::c_float,
                                    fx_left: *mut *mut ::libc::c_float,
                                    fx_right: *mut *mut ::libc::c_float)
     -> ::libc::c_int;
    pub fn fluid_synth_process(synth: *mut fluid_synth_t, len: ::libc::c_int,
                               nin: ::libc::c_int,
                               _in: *mut *mut ::libc::c_float,
                               nout: ::libc::c_int,
                               out: *mut *mut ::libc::c_float)
     -> ::libc::c_int;
    pub fn fluid_synth_add_sfloader(synth: *mut fluid_synth_t,
                                    loader: *mut fluid_sfloader_t) -> ();
    pub fn fluid_synth_alloc_voice(synth: *mut fluid_synth_t,
                                   sample: *mut fluid_sample_t,
                                   channum: ::libc::c_int, key: ::libc::c_int,
                                   vel: ::libc::c_int) -> *mut fluid_voice_t;
    pub fn fluid_synth_start_voice(synth: *mut fluid_synth_t,
                                   voice: *mut fluid_voice_t) -> ();
    pub fn fluid_synth_get_voicelist(synth: *mut fluid_synth_t,
                                     buf: *mut *mut fluid_voice_t,
                                     bufsize: ::libc::c_int,
                                     ID: ::libc::c_int) -> ();
    pub fn fluid_synth_handle_midi_event(data: *mut ::libc::c_void,
                                         event: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_synth_set_midi_router(synth: *mut fluid_synth_t,
                                       router: *mut fluid_midi_router_t)
     -> ();
    pub fn fluid_get_stdin() -> fluid_istream_t;
    pub fn fluid_get_stdout() -> fluid_ostream_t;
    pub fn fluid_get_userconf(buf: *mut ::libc::c_char, len: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn fluid_get_sysconf(buf: *mut ::libc::c_char, len: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn new_fluid_cmd_handler(synth: *mut fluid_synth_t)
     -> *mut fluid_cmd_handler_t;
    pub fn delete_fluid_cmd_handler(handler: *mut fluid_cmd_handler_t) -> ();
    pub fn fluid_cmd_handler_set_synth(handler: *mut fluid_cmd_handler_t,
                                       synth: *mut fluid_synth_t) -> ();
    pub fn fluid_cmd_handler_register(handler: *mut fluid_cmd_handler_t,
                                      cmd: *mut fluid_cmd_t) -> ::libc::c_int;
    pub fn fluid_cmd_handler_unregister(handler: *mut fluid_cmd_handler_t,
                                        cmd: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_command(handler: *mut fluid_cmd_handler_t,
                         cmd: *const ::libc::c_char, out: fluid_ostream_t)
     -> ::libc::c_int;
    pub fn fluid_source(handler: *mut fluid_cmd_handler_t,
                        filename: *const ::libc::c_char) -> ::libc::c_int;
    pub fn fluid_usershell(settings: *mut fluid_settings_t,
                           handler: *mut fluid_cmd_handler_t) -> ();
    pub fn new_fluid_shell(settings: *mut fluid_settings_t,
                           handler: *mut fluid_cmd_handler_t,
                           _in: fluid_istream_t, out: fluid_ostream_t,
                           thread: ::libc::c_int) -> *mut fluid_shell_t;
    pub fn delete_fluid_shell(shell: *mut fluid_shell_t) -> ();
    pub fn new_fluid_server(settings: *mut fluid_settings_t,
                            func: fluid_server_newclient_func_t,
                            data: *mut ::libc::c_void) -> *mut fluid_server_t;
    pub fn delete_fluid_server(server: *mut fluid_server_t) -> ();
    pub fn fluid_server_join(server: *mut fluid_server_t) -> ::libc::c_int;
    pub fn fluid_ramsfont_create_sfont() -> *mut fluid_sfont_t;
    pub fn fluid_ramsfont_set_name(sfont: *mut fluid_ramsfont_t,
                                   name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_ramsfont_add_izone(sfont: *mut fluid_ramsfont_t,
                                    bank: ::libc::c_uint, num: ::libc::c_uint,
                                    sample: *mut fluid_sample_t,
                                    lokey: ::libc::c_int,
                                    hikey: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_ramsfont_remove_izone(sfont: *mut fluid_ramsfont_t,
                                       bank: ::libc::c_uint,
                                       num: ::libc::c_uint,
                                       sample: *mut fluid_sample_t)
     -> ::libc::c_int;
    pub fn fluid_ramsfont_izone_set_gen(sfont: *mut fluid_ramsfont_t,
                                        bank: ::libc::c_uint,
                                        num: ::libc::c_uint,
                                        sample: *mut fluid_sample_t,
                                        gen_type: ::libc::c_int,
                                        value: ::libc::c_float)
     -> ::libc::c_int;
    pub fn fluid_ramsfont_izone_set_loop(sfont: *mut fluid_ramsfont_t,
                                         bank: ::libc::c_uint,
                                         num: ::libc::c_uint,
                                         sample: *mut fluid_sample_t,
                                         on: ::libc::c_int,
                                         loopstart: ::libc::c_float,
                                         loopend: ::libc::c_float)
     -> ::libc::c_int;
    pub fn new_fluid_ramsample() -> *mut fluid_sample_t;
    pub fn delete_fluid_ramsample(sample: *mut fluid_sample_t)
     -> ::libc::c_int;
    pub fn fluid_sample_set_name(sample: *mut fluid_sample_t,
                                 name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_sample_set_sound_data(sample: *mut fluid_sample_t,
                                       data: *mut ::libc::c_short,
                                       nbframes: ::libc::c_uint,
                                       copy_data: ::libc::c_short,
                                       rootkey: ::libc::c_int)
     -> ::libc::c_int;
    pub fn new_fluid_audio_driver(settings: *mut fluid_settings_t,
                                  synth: *mut fluid_synth_t)
     -> *mut fluid_audio_driver_t;
    pub fn new_fluid_audio_driver2(settings: *mut fluid_settings_t,
                                   func: fluid_audio_func_t,
                                   data: *mut ::libc::c_void)
     -> *mut fluid_audio_driver_t;
    pub fn delete_fluid_audio_driver(driver: *mut fluid_audio_driver_t) -> ();
    pub fn new_fluid_file_renderer(synth: *mut fluid_synth_t)
     -> *mut fluid_file_renderer_t;
    pub fn fluid_file_renderer_process_block(dev: *mut fluid_file_renderer_t)
     -> ::libc::c_int;
    pub fn delete_fluid_file_renderer(dev: *mut fluid_file_renderer_t) -> ();
    pub fn new_fluid_event() -> *mut fluid_event_t;
    pub fn delete_fluid_event(evt: *mut fluid_event_t) -> ();
    pub fn fluid_event_set_source(evt: *mut fluid_event_t,
                                  src: ::libc::c_short) -> ();
    pub fn fluid_event_set_dest(evt: *mut fluid_event_t,
                                dest: ::libc::c_short) -> ();
    pub fn fluid_event_timer(evt: *mut fluid_event_t,
                             data: *mut ::libc::c_void) -> ();
    pub fn fluid_event_note(evt: *mut fluid_event_t, channel: ::libc::c_int,
                            key: ::libc::c_short, vel: ::libc::c_short,
                            duration: ::libc::c_uint) -> ();
    pub fn fluid_event_noteon(evt: *mut fluid_event_t, channel: ::libc::c_int,
                              key: ::libc::c_short, vel: ::libc::c_short)
     -> ();
    pub fn fluid_event_noteoff(evt: *mut fluid_event_t,
                               channel: ::libc::c_int, key: ::libc::c_short)
     -> ();
    pub fn fluid_event_all_sounds_off(evt: *mut fluid_event_t,
                                      channel: ::libc::c_int) -> ();
    pub fn fluid_event_all_notes_off(evt: *mut fluid_event_t,
                                     channel: ::libc::c_int) -> ();
    pub fn fluid_event_bank_select(evt: *mut fluid_event_t,
                                   channel: ::libc::c_int,
                                   bank_num: ::libc::c_short) -> ();
    pub fn fluid_event_program_change(evt: *mut fluid_event_t,
                                      channel: ::libc::c_int,
                                      preset_num: ::libc::c_short) -> ();
    pub fn fluid_event_program_select(evt: *mut fluid_event_t,
                                      channel: ::libc::c_int,
                                      sfont_id: ::libc::c_uint,
                                      bank_num: ::libc::c_short,
                                      preset_num: ::libc::c_short) -> ();
    pub fn fluid_event_control_change(evt: *mut fluid_event_t,
                                      channel: ::libc::c_int,
                                      control: ::libc::c_short,
                                      val: ::libc::c_short) -> ();
    pub fn fluid_event_pitch_bend(evt: *mut fluid_event_t,
                                  channel: ::libc::c_int, val: ::libc::c_int)
     -> ();
    pub fn fluid_event_pitch_wheelsens(evt: *mut fluid_event_t,
                                       channel: ::libc::c_int,
                                       val: ::libc::c_short) -> ();
    pub fn fluid_event_modulation(evt: *mut fluid_event_t,
                                  channel: ::libc::c_int,
                                  val: ::libc::c_short) -> ();
    pub fn fluid_event_sustain(evt: *mut fluid_event_t,
                               channel: ::libc::c_int, val: ::libc::c_short)
     -> ();
    pub fn fluid_event_pan(evt: *mut fluid_event_t, channel: ::libc::c_int,
                           val: ::libc::c_short) -> ();
    pub fn fluid_event_volume(evt: *mut fluid_event_t, channel: ::libc::c_int,
                              val: ::libc::c_short) -> ();
    pub fn fluid_event_reverb_send(evt: *mut fluid_event_t,
                                   channel: ::libc::c_int,
                                   val: ::libc::c_short) -> ();
    pub fn fluid_event_chorus_send(evt: *mut fluid_event_t,
                                   channel: ::libc::c_int,
                                   val: ::libc::c_short) -> ();
    pub fn fluid_event_channel_pressure(evt: *mut fluid_event_t,
                                        channel: ::libc::c_int,
                                        val: ::libc::c_short) -> ();
    pub fn fluid_event_system_reset(evt: *mut fluid_event_t) -> ();
    pub fn fluid_event_any_control_change(evt: *mut fluid_event_t,
                                          channel: ::libc::c_int) -> ();
    pub fn fluid_event_unregistering(evt: *mut fluid_event_t) -> ();
    pub fn fluid_event_get_type(evt: *mut fluid_event_t) -> ::libc::c_int;
    pub fn fluid_event_get_source(evt: *mut fluid_event_t) -> ::libc::c_short;
    pub fn fluid_event_get_dest(evt: *mut fluid_event_t) -> ::libc::c_short;
    pub fn fluid_event_get_channel(evt: *mut fluid_event_t) -> ::libc::c_int;
    pub fn fluid_event_get_key(evt: *mut fluid_event_t) -> ::libc::c_short;
    pub fn fluid_event_get_velocity(evt: *mut fluid_event_t)
     -> ::libc::c_short;
    pub fn fluid_event_get_control(evt: *mut fluid_event_t)
     -> ::libc::c_short;
    pub fn fluid_event_get_value(evt: *mut fluid_event_t) -> ::libc::c_short;
    pub fn fluid_event_get_program(evt: *mut fluid_event_t)
     -> ::libc::c_short;
    pub fn fluid_event_get_data(evt: *mut fluid_event_t)
     -> *mut ::libc::c_void;
    pub fn fluid_event_get_duration(evt: *mut fluid_event_t)
     -> ::libc::c_uint;
    pub fn fluid_event_get_bank(evt: *mut fluid_event_t) -> ::libc::c_short;
    pub fn fluid_event_get_pitch(evt: *mut fluid_event_t) -> ::libc::c_int;
    pub fn fluid_event_get_sfont_id(evt: *mut fluid_event_t)
     -> ::libc::c_uint;
    pub fn new_fluid_midi_event() -> *mut fluid_midi_event_t;
    pub fn delete_fluid_midi_event(event: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_type(evt: *mut fluid_midi_event_t,
                                     _type: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_type(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_channel(evt: *mut fluid_midi_event_t,
                                        chan: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_channel(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_get_key(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_key(evt: *mut fluid_midi_event_t,
                                    key: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_velocity(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_velocity(evt: *mut fluid_midi_event_t,
                                         vel: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_control(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_control(evt: *mut fluid_midi_event_t,
                                        ctrl: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_value(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_value(evt: *mut fluid_midi_event_t,
                                      val: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_program(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_program(evt: *mut fluid_midi_event_t,
                                        val: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_get_pitch(evt: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_event_set_pitch(evt: *mut fluid_midi_event_t,
                                      val: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_midi_event_set_sysex(evt: *mut fluid_midi_event_t,
                                      data: *mut ::libc::c_void,
                                      size: ::libc::c_int,
                                      dynamic: ::libc::c_int)
     -> ::libc::c_int;
    pub fn new_fluid_midi_router(settings: *mut fluid_settings_t,
                                 callback: fn (closure: *mut ::libc::c_void, event: *mut fluid_midi_event_t) -> i32,
                                 event_handler_data: *mut ::libc::c_void)
     -> *mut fluid_midi_router_t;
    pub fn delete_fluid_midi_router(handler: *mut fluid_midi_router_t)
     -> ::libc::c_int;
    pub fn fluid_midi_router_set_default_rules(router:
                                                   *mut fluid_midi_router_t)
     -> ::libc::c_int;
    pub fn fluid_midi_router_clear_rules(router: *mut fluid_midi_router_t)
     -> ::libc::c_int;
    pub fn fluid_midi_router_add_rule(router: *mut fluid_midi_router_t,
                                      rule: *mut fluid_midi_router_rule_t,
                                      _type: ::libc::c_int) -> ::libc::c_int;
    pub fn new_fluid_midi_router_rule() -> *mut fluid_midi_router_rule_t;
    pub fn delete_fluid_midi_router_rule(rule: *mut fluid_midi_router_rule_t)
     -> ();
    pub fn fluid_midi_router_rule_set_chan(rule:
                                               *mut fluid_midi_router_rule_t,
                                           min: ::libc::c_int,
                                           max: ::libc::c_int,
                                           mul: ::libc::c_float,
                                           add: ::libc::c_int) -> ();
    pub fn fluid_midi_router_rule_set_param1(rule:
                                                 *mut fluid_midi_router_rule_t,
                                             min: ::libc::c_int,
                                             max: ::libc::c_int,
                                             mul: ::libc::c_float,
                                             add: ::libc::c_int) -> ();
    pub fn fluid_midi_router_rule_set_param2(rule:
                                                 *mut fluid_midi_router_rule_t,
                                             min: ::libc::c_int,
                                             max: ::libc::c_int,
                                             mul: ::libc::c_float,
                                             add: ::libc::c_int) -> ();
    pub fn fluid_midi_router_handle_midi_event(data: *mut ::libc::c_void,
                                               event: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_dump_prerouter(data: *mut ::libc::c_void,
                                     event: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_midi_dump_postrouter(data: *mut ::libc::c_void,
                                      event: *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn new_fluid_midi_driver(settings: *mut fluid_settings_t,
                                 handler: fn (closure: *mut ::libc::c_void, event: *mut fluid_midi_event_t) -> i32,
                                 event_handler_data: *mut ::libc::c_void)
     -> *mut fluid_midi_driver_t;
    pub fn delete_fluid_midi_driver(driver: *mut fluid_midi_driver_t) -> ();
    pub fn new_fluid_player(synth: *mut fluid_synth_t) -> *mut fluid_player_t;
    pub fn delete_fluid_player(player: *mut fluid_player_t) -> ::libc::c_int;
    pub fn fluid_player_add(player: *mut fluid_player_t,
                            midifile: *const ::libc::c_char) -> ::libc::c_int;
    pub fn fluid_player_add_mem(player: *mut fluid_player_t,
                                buffer: *const ::libc::c_void, len: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn fluid_player_play(player: *mut fluid_player_t) -> ::libc::c_int;
    pub fn fluid_player_stop(player: *mut fluid_player_t) -> ::libc::c_int;
    pub fn fluid_player_join(player: *mut fluid_player_t) -> ::libc::c_int;
    pub fn fluid_player_set_loop(player: *mut fluid_player_t,
                                 _loop: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_player_set_midi_tempo(player: *mut fluid_player_t,
                                       tempo: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_player_set_bpm(player: *mut fluid_player_t,
                                bpm: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_player_get_status(player: *mut fluid_player_t)
     -> ::libc::c_int;
    pub fn fluid_player_set_playback_callback(player: *mut fluid_player_t,
                                              handler:
                                                  handle_midi_event_func_t,
                                              handler_data:
                                                  *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn new_fluid_sequencer() -> *mut fluid_sequencer_t;
    pub fn new_fluid_sequencer2(use_system_timer: ::libc::c_int)
     -> *mut fluid_sequencer_t;
    pub fn delete_fluid_sequencer(seq: *mut fluid_sequencer_t) -> ();
    pub fn fluid_sequencer_get_use_system_timer(seq: *mut fluid_sequencer_t)
     -> ::libc::c_int;
    pub fn fluid_sequencer_register_client(seq: *mut fluid_sequencer_t,
                                           name: *const ::libc::c_char,
                                           callback: fn (closure: *mut ::libc::c_void, time: ::libc::c_uint, event: *mut fluid_event_t, seq: *mut fluid_sequencer_t),
                                           data: *mut ::libc::c_void)
     -> ::libc::c_short;
    pub fn fluid_sequencer_unregister_client(seq: *mut fluid_sequencer_t,
                                             id: ::libc::c_short) -> ();
    pub fn fluid_sequencer_count_clients(seq: *mut fluid_sequencer_t)
     -> ::libc::c_int;
    pub fn fluid_sequencer_get_client_id(seq: *mut fluid_sequencer_t,
                                         index: ::libc::c_int)
     -> ::libc::c_short;
    pub fn fluid_sequencer_get_client_name(seq: *mut fluid_sequencer_t,
                                           id: ::libc::c_int)
     -> *mut ::libc::c_char;
    pub fn fluid_sequencer_client_is_dest(seq: *mut fluid_sequencer_t,
                                          id: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_sequencer_process(seq: *mut fluid_sequencer_t,
                                   msec: ::libc::c_uint) -> ();
    pub fn fluid_sequencer_send_now(seq: *mut fluid_sequencer_t,
                                    evt: *mut fluid_event_t) -> ();
    pub fn fluid_sequencer_send_at(seq: *mut fluid_sequencer_t,
                                   evt: *mut fluid_event_t,
                                   time: ::libc::c_uint,
                                   absolute: ::libc::c_int) -> ::libc::c_int;
    pub fn fluid_sequencer_remove_events(seq: *mut fluid_sequencer_t,
                                         source: ::libc::c_short,
                                         dest: ::libc::c_short,
                                         _type: ::libc::c_int) -> ();
    pub fn fluid_sequencer_get_tick(seq: *mut fluid_sequencer_t)
     -> ::libc::c_uint;
    pub fn fluid_sequencer_set_time_scale(seq: *mut fluid_sequencer_t,
                                          scale: ::libc::c_double) -> ();
    pub fn fluid_sequencer_get_time_scale(seq: *mut fluid_sequencer_t)
     -> ::libc::c_double;
    pub fn fluid_sequencer_register_fluidsynth(seq: *mut fluid_sequencer_t,
                                               synth: *mut fluid_synth_t)
     -> ::libc::c_short;
    pub fn fluid_sequencer_add_midi_event_to_buffer(data: *mut ::libc::c_void,
                                                    event:
                                                        *mut fluid_midi_event_t)
     -> ::libc::c_int;
    pub fn fluid_set_log_function(level: ::libc::c_int,
                                  fun: fluid_log_function_t,
                                  data: *mut ::libc::c_void)
     -> fluid_log_function_t;
    pub fn fluid_default_log_function(level: ::libc::c_int,
                                      message: *mut ::libc::c_char,
                                      data: *mut ::libc::c_void) -> ();
    pub fn fluid_log(level: ::libc::c_int, fmt: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn fluid_is_soundfont(filename: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_is_midifile(filename: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn fluid_mod_new() -> *mut fluid_mod_t;
    pub fn fluid_mod_delete(_mod: *mut fluid_mod_t) -> ();
    pub fn fluid_mod_set_source1(_mod: *mut fluid_mod_t, src: ::libc::c_int,
                                 flags: ::libc::c_int) -> ();
    pub fn fluid_mod_set_source2(_mod: *mut fluid_mod_t, src: ::libc::c_int,
                                 flags: ::libc::c_int) -> ();
    pub fn fluid_mod_set_dest(_mod: *mut fluid_mod_t, dst: ::libc::c_int)
     -> ();
    pub fn fluid_mod_set_amount(_mod: *mut fluid_mod_t,
                                amount: ::libc::c_double) -> ();
    pub fn fluid_mod_get_source1(_mod: *mut fluid_mod_t) -> ::libc::c_int;
    pub fn fluid_mod_get_flags1(_mod: *mut fluid_mod_t) -> ::libc::c_int;
    pub fn fluid_mod_get_source2(_mod: *mut fluid_mod_t) -> ::libc::c_int;
    pub fn fluid_mod_get_flags2(_mod: *mut fluid_mod_t) -> ::libc::c_int;
    pub fn fluid_mod_get_dest(_mod: *mut fluid_mod_t) -> ::libc::c_int;
    pub fn fluid_mod_get_amount(_mod: *mut fluid_mod_t) -> ::libc::c_double;
    pub fn fluid_mod_test_identity(mod1: *mut fluid_mod_t,
                                   mod2: *mut fluid_mod_t) -> ::libc::c_int;
    pub fn fluid_gen_set_default_values(gen: *mut fluid_gen_t)
     -> ::libc::c_int;
    pub fn fluid_voice_update_param(voice: *mut fluid_voice_t,
                                    gen: ::libc::c_int) -> ();
    pub fn fluid_voice_add_mod(voice: *mut fluid_voice_t,
                               _mod: *mut fluid_mod_t, mode: ::libc::c_int)
     -> ();
    pub fn fluid_voice_gen_set(voice: *mut fluid_voice_t, gen: ::libc::c_int,
                               val: ::libc::c_float) -> ();
    pub fn fluid_voice_gen_get(voice: *mut fluid_voice_t, gen: ::libc::c_int)
     -> ::libc::c_float;
    pub fn fluid_voice_gen_incr(voice: *mut fluid_voice_t, gen: ::libc::c_int,
                                val: ::libc::c_float) -> ();
    pub fn fluid_voice_get_id(voice: *mut fluid_voice_t) -> ::libc::c_uint;
    pub fn fluid_voice_is_playing(voice: *mut fluid_voice_t) -> ::libc::c_int;
    pub fn fluid_voice_optimize_sample(s: *mut fluid_sample_t)
     -> ::libc::c_int;
    pub fn fluid_version(major: *mut ::libc::c_int, minor: *mut ::libc::c_int,
                         micro: *mut ::libc::c_int) -> ();
    pub fn fluid_version_str() -> *mut ::libc::c_char;
}

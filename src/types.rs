extern crate libc;
use libc::{c_int, c_char, c_float, c_double, c_uint, c_uchar};

#[repr(i32)]
pub enum fluid_gen_type {
  GEN_STARTADDROFS = 0,		
  GEN_ENDADDROFS = 1,		
  GEN_STARTLOOPADDROFS = 2,		
  GEN_ENDLOOPADDROFS = 3,		
  GEN_STARTADDRCOARSEOFS = 4,	
  GEN_MODLFOTOPITCH = 5,		
  GEN_VIBLFOTOPITCH = 6,		
  GEN_MODENVTOPITCH = 7,		
  GEN_FILTERFC = 8,			
  GEN_FILTERQ = 9,			
  GEN_MODLFOTOFILTERFC = 10,		
  GEN_MODENVTOFILTERFC = 11,		
  GEN_ENDADDRCOARSEOFS = 12,		
  GEN_MODLFOTOVOL = 13, 		
  GEN_UNUSED1 = 14,			
  GEN_CHORUSSEND = 15,		
  GEN_REVERBSEND = 16,		
  GEN_PAN = 17,			
  GEN_UNUSED2 = 18,			
  GEN_UNUSED3 = 19,			
  GEN_UNUSED4 = 20,			
  GEN_MODLFODELAY = 21,		
  GEN_MODLFOFREQ = 22,		
  GEN_VIBLFODELAY = 23,		
  GEN_VIBLFOFREQ = 24,		
  GEN_MODENVDELAY = 25,		
  GEN_MODENVATTACK = 26,		
  GEN_MODENVHOLD = 27,		
  GEN_MODENVDECAY = 28,		
  GEN_MODENVSUSTAIN = 29,		
  GEN_MODENVRELEASE = 30,		
  GEN_KEYTOMODENVHOLD = 31,		
  GEN_KEYTOMODENVDECAY = 32,		
  GEN_VOLENVDELAY = 33,		
  GEN_VOLENVATTACK = 34,		
  GEN_VOLENVHOLD = 35,		
  GEN_VOLENVDECAY = 36,		
  GEN_VOLENVSUSTAIN = 37,		
  GEN_VOLENVRELEASE = 38,		
  GEN_KEYTOVOLENVHOLD = 39,		
  GEN_KEYTOVOLENVDECAY = 40, 		
  GEN_INSTRUMENT = 41,		
  GEN_RESERVED1 = 42,		
  GEN_KEYRANGE = 43,			
  GEN_VELRANGE = 44,			
  GEN_STARTLOOPADDRCOARSEOFS = 45,	
  GEN_KEYNUM = 46,			
  GEN_VELOCITY = 47,			
  GEN_ATTENUATION = 48,		
  GEN_RESERVED2 = 49,		
  GEN_ENDLOOPADDRCOARSEOFS = 50,	
  GEN_COARSETUNE = 51,		
  GEN_FINETUNE = 52,			
  GEN_SAMPLEID = 53,			
  GEN_SAMPLEMODE = 54,		
  GEN_RESERVED3 = 55,		
  GEN_SCALETUNE = 56,		
  GEN_EXCLUSIVECLASS = 57,		
  GEN_OVERRIDEROOTKEY = 58,		
  GEN_PITCH = 59,			
  GEN_LAST = 60,			
}

pub const FLUID_NUM_MOD: c_uint = 64;


#[repr(C)]
pub struct fluid_channel_t {
    pub mutex: fluid_mutex_t,

    pub synth: *mut fluid_synth_t,

}

#[repr(C)]
pub struct fluid_voice_t { 
    pub id: c_uint,
    pub status: c_uchar,
    pub key: c_uchar,
    pub vel: c_uchar,
    pub channel: *mut fluid_channel_t,
    pub gen: [fluid_gen_t, ..fluid_gen_type::GEN_LAST],
    pub _mod: [fluid_mod_t, ..FLUID_NUM_MOD]  
    pub mod_count: c_int,
    pub sample: *mut fluid_sample_t,

    pub has_noteoff: c_int,

    pub output_rate: fluid_real_t,

    pub start_time: c_uint,
    pub volenv: fluid_adsr_env_t,

    pub pitch: fluid_real_t,
    pub attenuation: fluid_real_t,
    pub root_pitch: fluid_real_t,

    pub synth_gain: fluid_real_t,

    pub pan: fluid_real_t,
    pub amp_left: fluid_real_t,
    pub amp_right: fluid_real_t,

    pub reverb_send: fluid_real_t,
    pub amp_reverb: fluid_real_t,

    pub chorus_send: fluid_real_t,
    pub amp_chorus: fluid_real_t,

    pub rvoice: *mut fluid_rvoice_t,
    pub overflow_rvoice: *mut fluid_rvoice_t,
    pub can_access_rvoice: c_int,
    pub can_access_overflow_rvoice: c_int,

    pub debug: c_int,
    pub _ref: c_double,
}


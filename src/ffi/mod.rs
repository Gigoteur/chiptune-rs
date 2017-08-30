#![allow(non_camel_case_types, non_snake_case, dead_code)]

use libc::{c_void, c_int, c_char, c_short, c_ushort, c_uint, c_uchar};

pub const MUS_INSTRUMENT_NAME_LEN : c_int = 32;
pub const MUS_PROG_LEN: c_int = 32;

pub const FREQ_TAB_SIZE : c_int = 96;
pub const MAX_VOLUME: c_int = 128;
pub const CYD_WAVE_MAX_ENTRIES: c_int = 128;
pub const CYD_MAX_FX_CHANNELS: c_int = 8;
pub const CYD_CUTOFF_MAX: c_int = 2048;

pub const MUS_FX_ARPEGGIO : c_int = 0x0000;
pub const MUS_FX_ARPEGGIO_ABS : c_int = 0x4000;
pub const MUS_FX_SET_EXT_ARP : c_int = 0x1000;
pub const MUS_FX_PORTA_UP : c_int = 0x0100;
pub const MUS_FX_PORTA_DN : c_int = 0x0200;
pub const MUS_FX_PORTA_UP_LOG : c_int = 0x0500;
pub const MUS_FX_PORTA_DN_LOG : c_int = 0x0600;
pub const MUS_FX_SLIDE : c_int = 0x0300;
pub const MUS_FX_VIBRATO : c_int = 0x0400;
pub const MUS_FX_FADE_VOLUME : c_int = 0x0a00;
pub const MUS_FX_SET_VOLUME : c_int = 0x0c00;
pub const MUS_FX_LOOP_PATTERN : c_int = 0x0d00;
pub const MUS_FX_SKIP_PATTERN : c_int = 0x2d00;
pub const MUS_FX_EXT : c_int = 0x0e00;
pub const MUS_FX_EXT_PORTA_UP : c_int = 0x0e10;
pub const MUS_FX_EXT_PORTA_DN : c_int = 0x0e20;
pub const MUS_FX_EXT_RETRIGGER : c_int = 0x0e90;
pub const MUS_FX_EXT_FADE_VOLUME_DN : c_int = 0x0ea0;
pub const MUS_FX_EXT_FADE_VOLUME_UP : c_int = 0x0eb0;
pub const MUS_FX_EXT_NOTE_CUT : c_int = 0x0ec0;
pub const MUS_FX_EXT_NOTE_DELAY : c_int = 0x0ed0;
pub const MUS_FX_SET_SPEED : c_int = 0x0f00;
pub const MUS_FX_SET_RATE : c_int = 0x1f00;
pub const MUS_FX_PORTA_UP_SEMI : c_int = 0x1100;
pub const MUS_FX_PORTA_DN_SEMI : c_int = 0x1200;
pub const MUS_FX_SET_PANNING : c_int = 0x1800;
pub const MUS_FX_PAN_LEFT : c_int = 0x1700;
pub const MUS_FX_PAN_RIGHT : c_int = 0x1900;
pub const MUS_FX_FADE_GLOBAL_VOLUME : c_int = 0x1a00;
pub const MUS_FX_SET_GLOBAL_VOLUME : c_int = 0x1d00;
pub const MUS_FX_SET_CHANNEL_VOLUME : c_int = 0x1c00;
pub const MUS_FX_CUTOFF_UP : c_int = 0x2100;
pub const MUS_FX_CUTOFF_DN : c_int = 0x2200;
pub const MUS_FX_CUTOFF_SET : c_int = 0x2900;
pub const MUS_FX_RESONANCE_SET : c_int = 0x2a00;
pub const MUS_FX_FILTER_TYPE : c_int = 0x2b00;
pub const MUS_FX_CUTOFF_SET_COMBINED : c_int = 0x2c00;
pub const MUS_FX_BUZZ_UP : c_int = 0x3100;
pub const MUS_FX_BUZZ_DN : c_int = 0x3200;
pub const MUS_FX_BUZZ_SHAPE : c_int = 0x3f00;
pub const MUS_FX_BUZZ_SET : c_int = 0x3900;
pub const MUS_FX_BUZZ_SET_SEMI : c_int = 0x3a00;
pub const MUS_FX_FM_SET_MODULATION : c_int = 0x3300;
pub const MUS_FX_FM_SET_FEEDBACK : c_int = 0x3400;
pub const MUS_FX_FM_SET_HARMONIC : c_int = 0x3500;
pub const MUS_FX_FM_SET_WAVEFORM : c_int = 0x3600;
pub const MUS_FX_PW_DN : c_int = 0x0700;
pub const MUS_FX_PW_UP : c_int = 0x0800;
pub const MUS_FX_PW_SET : c_int = 0x0900;
pub const MUS_FX_SET_WAVEFORM : c_int = 0x0b00;
pub const MUS_FX_SET_FXBUS : c_int = 0x1b00;
pub const MUS_FX_SET_SYNCSRC : c_int = 0x7a00;
pub const MUS_FX_SET_RINGSRC : c_int = 0x7b00;
pub const MUS_FX_SET_WAVETABLE_ITEM : c_int = 0x3b00;
pub const MUS_FX_SET_DOWNSAMPLE : c_int = 0x1e00;
pub const MUS_FX_WAVETABLE_OFFSET : c_int = 0x5000;
pub const MUS_FX_CUTOFF_FINE_SET : c_int = 0x6000;
pub const MUS_FX_END : c_int = 0xffff;
pub const MUS_FX_JUMP : c_int = 0xff00;
pub const MUS_FX_LABEL : c_int = 0xfd00;
pub const MUS_FX_LOOP : c_int = 0xfe00;
pub const MUS_FX_TRIGGER_RELEASE : c_int = 0x7c00;
pub const MUS_FX_RESTART_PROGRAM : c_int = 0x7d00;
pub const MUS_FX_NOP : c_int = 0xfffe;


#[repr(C)]
pub struct MusInstrument {
  pub flags: c_uint,
	pub cydflags: c_uint,
  pub musadsr_a: c_uchar,
  pub musadsr_d: c_uchar,
  pub musadsr_s: c_uchar,
  pub musadsr_r: c_uchar,
  pub sync_source: c_uchar,
  pub ring_mod: c_uchar,
  pub pw: c_ushort,
	pub volume: c_uchar,
  pub program: [c_ushort; 32],
	pub prog_period: c_uchar,
	pub vibrato_speed: c_uchar,
  pub vibrato_depth: c_uchar,
  pub slide_speed: c_uchar,
  pub pwm_speed: c_uchar,
  pub pwm_depth: c_uchar,
	pub base_note: c_uchar,
	pub cutoff: c_ushort,
	pub resonance: c_uchar,
	pub flttype: c_uchar,
	pub ym_env_shape: c_uchar,
	pub buzz_offset: c_short,
	pub fx_bus: c_uchar,
  pub vib_shape: c_uchar,
  pub vib_delay: c_uchar,
  pub pwm_shape: c_uchar,
	pub name: [c_char; 33],
	pub wavetable_entry: c_uchar,
	pub lfsr_type: c_uchar,
	pub finetune: c_char,
	pub fm_flags: c_uint,
	pub fm_modulation: c_uchar,
  pub fm_feedback: c_uchar,
  pub fm_wave: c_uchar,
  pub fm_harmonic: c_uchar,
	pub fm_adsr_a: c_uchar,
  pub fm_adsr_d: c_uchar,
  pub fm_adsr_s: c_uchar,
  pub fm_adsr_r: c_uchar,
	pub fm_attack_start: c_uchar,
}

pub type chiptune_player = *mut c_void;
pub type chiptune_song = *mut c_void;
pub type chiptune_sound = *mut MusInstrument;

extern {
  pub fn Chiptune_CreatePlayer(sample_rate: c_int) -> chiptune_player;
  pub fn Chiptune_LoadMusic(player: chiptune_player, path: *const c_char) -> chiptune_song;
  pub fn Chiptune_PlayMusic(player: chiptune_player, son: chiptune_song,  start_position: c_int);
  pub fn Chiptune_LoadSound(player: chiptune_player, path: *const c_char) -> chiptune_sound;
  pub fn Chiptune_PlaySound(player: chiptune_player, sound: chiptune_sound, chan: c_int, note: c_ushort, panning: c_int);
  pub fn Chiptune_Stop(player: chiptune_player);
  pub fn Chiptune_StopMusic(player: chiptune_player);
  pub fn Chiptune_StopSound(player: chiptune_player);
  pub fn Chiptune_Pause(player: chiptune_player, state: c_int);
  pub fn Chiptune_PauseMusic(player: chiptune_player, state: c_int);
  pub fn Chiptune_PauseSound(player: chiptune_player, state: c_int);
  pub fn Chiptune_SetPlayerQuality(player: chiptune_player, oversample: c_int);
  pub fn Chiptune_SetVolume(player: chiptune_player, volume: c_int);
  pub fn Chiptune_SetLooping(player: chiptune_player, looping: c_int);
  pub fn Chiptune_GetMusicPlayPosition(player: chiptune_player) -> c_int;
  pub fn Chiptune_GetSoundPlayPosition(player: chiptune_player) -> c_int;
  pub fn Chiptune_GetSongInfo(player: chiptune_player) -> c_int;
}

pub const CYD_PAN_CENTER : c_int = 64;
pub const CYD_PAN_LEFT : c_int = 0;
pub const CYD_PAN_RIGHT : c_int = 128;

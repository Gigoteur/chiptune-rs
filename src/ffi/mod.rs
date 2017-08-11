#![allow(non_camel_case_types, non_snake_case, dead_code)]

use libc::{c_void, c_int, c_char, c_short, c_ushort, c_uint, c_uchar};

pub const MUS_INSTRUMENT_NAME_LEN : c_int = 32;
pub const MUS_PROG_LEN: c_int = 32;

#[repr(C)]
pub struct MusInstrument {
  pub flags: c_uint,
	cydflags: c_uint,
  musadsr_a: c_uchar,
  musadsr_d: c_uchar,
  musadsr_s: c_uchar,
  musadsr_r: c_uchar,
  sync_source: c_uchar,
  ring_mod: c_uchar,
  pw: c_ushort,
	volume: c_uchar,
  pub program: [c_ushort; 32],
	prog_period: c_uchar,
	vibrato_speed: c_uchar,
  vibrato_depth: c_uchar,
  slide_speed: c_uchar,
  pwm_speed: c_uchar,
  pwm_depth: c_uchar,
	base_note: c_uchar,
	cutoff: c_ushort,
	resonance: c_uchar,
	flttype: c_uchar,
	ym_env_shape: c_uchar,
	buzz_offset: c_short,
	fx_bus: c_uchar,
  vib_shape: c_uchar,
  vib_delay: c_uchar,
  pwm_shape: c_uchar,
	pub name: [c_char; 33],
	wavetable_entry: c_uchar,
	lfsr_type: c_uchar,
	finetune: c_char,
	fm_flags: c_uint,
	fm_modulation: c_uchar,
  fm_feedback: c_uchar,
  fm_wave: c_uchar,
  fm_harmonic: c_uchar,
	fm_adsr_a: c_uchar,
  fm_adsr_d: c_uchar,
  fm_adsr_s: c_uchar,
  fm_adsr_r: c_uchar,
	fm_attack_start: c_uchar,
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

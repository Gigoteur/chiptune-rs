#![allow(non_camel_case_types, non_snake_case, dead_code)]

use libc::{c_void, c_int, c_char, c_ushort};

pub type chiptune_player = *mut c_void;
pub type chiptune_song = *mut c_void;
pub type chiptune_sound = *mut c_void;

extern {
  pub fn Chiptune_CreatePlayer(sample_rate: c_int) -> chiptune_player;
  pub fn Chiptune_LoadSong(player: chiptune_player, path: *const c_char) -> chiptune_song;
  pub fn Chiptune_PlaySong(player: chiptune_player, son: chiptune_song,  start_position: c_int);
  pub fn Chiptune_LoadSound(player: chiptune_player, path: *const c_char) -> chiptune_sound;
  pub fn Chiptune_PlaySound(player: chiptune_player, sound: chiptune_sound, chan: c_int, note: c_ushort, panning: c_int);
  pub fn Chiptune_Stop(player: chiptune_player);
  pub fn Chiptune_Pause(player: chiptune_player, state: c_int);
  pub fn Chiptune_SetPlayerQuality(player: chiptune_player, oversample: c_int);
  pub fn Chiptune_SetVolume(player: chiptune_player, volume: c_int);
  pub fn Chiptune_SetLooping(player: chiptune_player, looping: c_int);
  pub fn Chiptune_GetPlayPosition(player: chiptune_player) -> c_int;
  pub fn Chiptune_GetSongInfo(player: chiptune_player) -> c_int;
}

pub const CYD_PAN_CENTER : c_int = 64;
pub const CYD_PAN_LEFT : c_int = 0;
pub const CYD_PAN_RIGHT : c_int = 128;

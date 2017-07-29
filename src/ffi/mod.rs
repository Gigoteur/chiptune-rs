#![allow(non_camel_case_types, non_snake_case, dead_code)]

use libc::{c_void, c_int, c_char};

pub type chiptune_player = *mut c_void;
pub type chiptune_song = *mut c_void;
pub type chiptune_sound = *mut c_void;

extern {
  pub fn Chiptune_CreatePlayer(sample_rate: c_int) -> chiptune_player;
  pub fn Chiptune_LoadSong(player: chiptune_player, path: *const c_char) -> chiptune_song;
  pub fn Chiptune_PlaySong(player: chiptune_player, son: chiptune_song,  start_position: c_int);
  pub fn Chiptune_LoadSound(player: chiptune_player, path: *const c_char) -> chiptune_sound;
  pub fn Chiptune_PlaySound(player: chiptune_player, sound: chiptune_sound,  start_position: c_int);
}
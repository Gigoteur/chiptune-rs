pub mod ffi;

extern crate libc;
extern crate sdl2;

use std::ffi::{CString, CStr};
use libc::{c_void, c_int, c_char};
use sdl2::mixer;

#[allow(non_snake_case)]
pub struct Chiptune {
  P: ffi::chiptune_player,
}

#[allow(non_snake_case)]
pub struct ChiptuneSong {
  S: ffi::chiptune_song,
}

impl Chiptune {
  pub fn new() -> Chiptune {
    let _ = mixer::init(mixer::INIT_MP3 | mixer::INIT_FLAC | mixer::INIT_MOD |
                                mixer::INIT_FLUIDSYNTH |
                                mixer::INIT_MODPLUG |
                                mixer::INIT_OGG)
                    .unwrap();
    mixer::open_audio(mixer::DEFAULT_FREQUENCY,
                      mixer::DEFAULT_FORMAT,
                      mixer::DEFAULT_CHANNELS,
                      1024)
            .unwrap();


    unsafe {
      Chiptune { P: ffi::Chiptune_CreatePlayer(44100) }
    }
  }

  pub fn load_song(&mut self, path: String) -> ChiptuneSong {
    unsafe {
      let path = CString::new(path).unwrap();
      ChiptuneSong { S: ffi::Chiptune_LoadSong(self.P, path.as_ptr()) }
    }
  }

  pub fn play_song(&mut self, song: ChiptuneSong, start_position: c_int) {
    unsafe {
      ffi::Chiptune_PlaySong(self.P, song.S, start_position);
    }
  }

}
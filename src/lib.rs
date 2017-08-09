pub mod ffi;

extern crate libc;
extern crate sdl2;

use std::ffi::CString;
use libc::{c_int, c_ushort};
use sdl2::mixer;

#[derive(Debug)]
pub enum ChiptuneError {
    LoadingError,
}


#[allow(non_snake_case)]
pub struct Chiptune {
  P: ffi::chiptune_player,
}

#[allow(non_snake_case)]
pub struct ChiptuneSong {
  S: ffi::chiptune_song,
}

#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct ChiptuneSound {
  pub S: ffi::chiptune_sound,
}

pub use ffi::{
  CYD_PAN_CENTER, CYD_PAN_LEFT, CYD_PAN_RIGHT
};


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

  pub fn load_music(&mut self, path: String) -> Result<ChiptuneSong, ChiptuneError> {
    unsafe {
      let path = CString::new(path).unwrap();
      let s = ffi::Chiptune_LoadMusic(self.P, path.as_ptr());
      if s.is_null() {
        Err(ChiptuneError::LoadingError)
      } else {
        Ok(ChiptuneSong { S: s })
      }
    }
  }

  pub fn play_music(&mut self, song: &mut ChiptuneSong, start_position: c_int) {
    unsafe {
      ffi::Chiptune_PlayMusic(self.P, song.S, start_position);
    }
  }

  pub fn load_sound(&mut self, path: String) -> Result<ChiptuneSound, ChiptuneError> {
    unsafe {
      let path = CString::new(path).unwrap();
      let s = ffi::Chiptune_LoadSound(self.P, path.as_ptr());
      if s.is_null() {
        Err(ChiptuneError::LoadingError)
      } else {
        Ok(ChiptuneSound { S: s })
      }
    }
  }

  pub fn play_sound(&mut self, sound: &mut ChiptuneSound, chan: c_int, note: c_ushort, panning: c_int) {
    unsafe {
      ffi::Chiptune_PlaySound(self.P, sound.S, chan, note, panning);
    }
  }

  pub fn stop(&mut self) {
   unsafe {
      ffi::Chiptune_Stop(self.P);
    }
  }

  pub fn stop_music(&mut self) {
    unsafe {
      ffi::Chiptune_StopMusic(self.P);
    }
  }

  pub fn stop_sound(&mut self) {
    unsafe {
      ffi::Chiptune_StopSound(self.P);
    }
  }
  
  pub fn pause(&mut self, state: c_int) {
    unsafe {
      ffi::Chiptune_Pause(self.P, state);
    }
  }
  
  pub fn pause_music(&mut self, state: c_int) {
    unsafe {
      ffi::Chiptune_PauseMusic(self.P, state);
    }
  }

  pub fn pause_sound(&mut self, state: c_int) {
    unsafe {
      ffi::Chiptune_PauseSound(self.P, state);
    }
  }

  pub fn set_player_quality(&mut self, oversample: c_int) {
    unsafe {
      ffi::Chiptune_SetPlayerQuality(self.P, oversample);
    }
  }

  pub fn set_volume(&mut self, volume: c_int) {
    unsafe {
      ffi::Chiptune_SetVolume(self.P, volume);
    }
  }

  pub fn set_looping(&mut self, looping: c_int) {
    unsafe {
      ffi::Chiptune_SetLooping(self.P, looping);
    }
  }

  pub fn get_position(&mut self) -> c_int {
    unsafe {
      return ffi::Chiptune_GetPlayPosition(self.P);
    }
  }

  pub fn get_sound_program(&mut self, sound: ChiptuneSound) -> [u16; 32] {
    let mut program: [u16; 32] = [0; 32];
    
    unsafe {
        for i in 0..32 {
          program[i] = (*sound.S).program[i];
        }
    }
    program
  }
  
}
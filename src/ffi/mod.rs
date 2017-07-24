#![allow(non_camel_case_types, non_snake_case, dead_code)]

use libc::c_int;

extern {
  pub fn Chiptune_CreatePlayer(sample_rate: c_int) -> c_int;
}
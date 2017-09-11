extern crate chiptune;

use std::thread;
use std::time::Duration;

fn play_sound(player: &mut chiptune::Chiptune, path: String) -> Result<chiptune::ChiptuneSound, chiptune::ChiptuneError> {
    let sound = player.load_sound(path);
    match sound {
        Ok(mut chip_sound) => {
            println!("Playing sound");
            player.play_sound(&mut chip_sound, -1, 13312, chiptune::CYD_PAN_CENTER, 50);
        }
        Err(e) => println!("ERROR {:?}", e),
    }

    sound
}

fn main() {
    let mut player = chiptune::Chiptune::new();

    println!("Play music");
    let song = player.load_music("libksnd-source/src/assets/ringmod.kt".to_string());
    match song {
        Ok(mut chip_song) => {
            //player.play_music(&mut chip_song, 0);
        }
        Err(e) => println!("ERROR {:?}", e),
    }
    thread::sleep(Duration::from_secs(1));

    println!("SOUND POSITION = {:?}", player.get_sound_position());

    println!("Play sound");
    let sound = play_sound(&mut player, "libksnd-source/src/assets/sounds/major.ki".to_string());
    match sound {
        Ok(mut chip_sound) => {
            let program = player.get_sound_program(chip_sound);
            for i in 0..32 {
                println!("Program[{:?}] {:X}", i, program[i]);
                match chiptune::get_instruction(program[i] as i32) {
                    Ok(v) => {
                        //println!("{:?}", v);
                        match chiptune::notename(program[i] as i32, player.get_base_note(chip_sound)) {
                            Ok(name) => println!("NAME {:?}", name),
                            Err(_) => (),
                        }
                    },
                    Err(e) => println!("Error {:?}", e),
                }

            }
        }
        Err(e) => println!("ERROR {:?}", e),
    }

    thread::sleep(Duration::from_secs(5));
    println!("CLAP");
    play_sound(&mut player, "libksnd-source/src/assets/sounds/clap.ki".to_string());

    println!("SOUND POSITION = {:?}", player.get_sound_position());

    println!("MUSIC POSITION = {:?}", player.get_music_position());
    player.set_volume(64);

    thread::sleep(Duration::from_secs(10));

    println!("MUSIC POSITION = {:?}", player.get_music_position());
}
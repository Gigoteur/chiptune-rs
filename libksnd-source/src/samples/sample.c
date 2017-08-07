#include "chiptune.h"
#include "music.h"

#include <SDL2/SDL_mixer.h>


int main(int argc, char *argv[]) {

    ChiptunePlayer *player;
	ChiptuneSong * song;
	ChiptuneSound * sound;

    if(Mix_OpenAudio(44100, MIX_DEFAULT_FORMAT, MIX_DEFAULT_CHANNELS, 1024) == -1)
    {
      printf("%s", Mix_GetError());
      return -1;
    }

    printf("CREATE PLAYER\n");
	player = Chiptune_CreatePlayer(44100);
	printf("[E] CREATE PLAYER\n");

    printf("Load Song\n");
	song = Chiptune_LoadSong(player, "./assets/ringmod.kt");
    printf("[E] Load Song\n");

    printf("Play Song\n");	
	Chiptune_PlaySong(player, song, 0);
	printf("[E] Play Song\n");

    printf("Load Sound\n");
    sound = Chiptune_LoadSound(player, "./assets/sounds/the_horror.ki");
    printf("[E] Load Sound %x\n", sound);
    Chiptune_GetSoundInfo(sound);

	sleep(1);

    printf("Play Sound\n");	
	Chiptune_PlaySound(player, sound, -1, 10000, CYD_PAN_CENTER);
	printf("[E] Play Sound\n");

    sleep(5);
	
    printf("Change volume\n");	

    Chiptune_SetVolume(player, 10);

    sleep(5);

    Chiptune_SetVolume(player, 128);

    printf("Pause\n");	
	Chiptune_Pause(player, 1);
	
	sleep(2);
	
	Chiptune_Pause(player, 0);
	
	sleep(1);
		
	Chiptune_FreePlayer(player);
	Chiptune_FreeSong(song);
		
	return 0;
}
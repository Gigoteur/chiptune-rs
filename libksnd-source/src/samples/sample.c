#include "chiptune.h"

#include <SDL2/SDL_mixer.h>


int main(int argc, char *argv[]) {

    ChiptunePlayer *player;
	ChiptuneSong * song;
	ChiptuneSound * sound;

    if(Mix_OpenAudio(44100, MIX_DEFAULT_FORMAT, MIX_DEFAULT_CHANNELS, 1024) == -1) //Initialisation de l'API Mixer
    {
      printf("%s", Mix_GetError());
    }

    printf("CREATE PLAYER\n");
	player = KSND_CreatePlayer(44100);
	printf("[E] CREATE PLAYER\n");

    printf("Load Song\n");
	song = KSND_LoadSong(player, "./assets/ringmod.kt");
    printf("[E] Load Song\n");

    printf("Play Song\n");	
	KSND_PlaySong(player, song, 0);
	printf("[E] Play Song\n");

    printf("Load Sound\n");
    sound = KSND_LoadSound(player, "./assets/sounds/clap.ki");
    printf("[E] Load Sound %x\n", sound);

	sleep(2);

    printf("Play Sound\n");	
	KSND_PlaySound(player, sound, 0);
	printf("[E] Play Sound\n");

    sleep(5);
	
    KSND_SetVolume(player, 128);

	KSND_Pause(player, 1);
	
	sleep(2);
	
	KSND_Pause(player, 0);
	
	sleep(1);
		
	KSND_FreePlayer(player);
	KSND_FreeSong(song);
		
	return 0;

    return 0;
}
#include "chiptune.h"
#include "music.h"

#include <SDL2/SDL_mixer.h>


int main(int argc, char *argv[]) {

    ChiptunePlayer *player;
	ChiptuneSong *song;
	ChiptuneSound *sound, *sound2;

    if(Mix_OpenAudio(44100, MIX_DEFAULT_FORMAT, MIX_DEFAULT_CHANNELS, 1024) == -1)
    {
      printf("%s", Mix_GetError());
      return -1;
    }

    printf("CREATE PLAYER\n");
	player = Chiptune_CreatePlayer(44100);
	printf("[E] CREATE PLAYER\n");

    printf("Load Music\n");
	song = Chiptune_LoadMusic(player, "./assets/ringmod.kt");
    printf("[E] Load Music\n");

    printf("Play Music\n");	
	Chiptune_PlayMusic(player, song, 0);
	printf("[E] Play Music\n");

    printf("Load Sound\n");
    sound = Chiptune_LoadSound(player, "./assets/sounds/the_horror.ki");
    sound2 = Chiptune_LoadSound(player, "./assets/sounds/clap.ki");
    
    printf("[E] Load Sound %x\n", sound);

	sleep(1);
    printf("MUSIC POSITION %d\n", Chiptune_GetMusicPlayPosition(player));

    printf("Play Sound\n");	
	Chiptune_PlaySound(player, sound, 0, 10000, CYD_PAN_CENTER, 50);
	printf("[E] Play Sound\n");
    printf("SOUND POSITION %d\n", Chiptune_GetSoundPlayPosition(player, 0));
    
    printf("SOUND POSITION %d\n", Chiptune_GetSoundPlayPosition(player, 0));
    
    sleep(1);

    printf("Change volume\n");	    
    Chiptune_SetVolume(player, 64);    

    printf("SOUND POSITION %d\n", Chiptune_GetSoundPlayPosition(player, 0));
    
    printf("Pause sound\n");	
	Chiptune_PauseMusic(player, 1);
    sleep(3);
    Chiptune_PlaySound(player, sound2, 1, 1, CYD_PAN_CENTER, 50);
    
    printf("Resume sound\n");	
	Chiptune_PauseMusic(player, 0);

    Chiptune_SetVolume(player, 128);

	sleep(5);

    Chiptune_Stop(player);
    printf("Stop sound\n");	

    sleep(1);

	Chiptune_FreePlayer(player);
	Chiptune_FreeSong(song);
		
	return 0;
}
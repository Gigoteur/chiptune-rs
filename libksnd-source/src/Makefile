CC= gcc -std=gnu99
CFLAGS= -O2 -Wall -Wextra

ECHO = echo
MKDIR= mkdir
AR= ar rcu
RANLIB= ranlib
RM= rm -f

SDLFLAGS = `sdl2-config --cflags` -U_FORTIFY_SOURCE
INCLUDEFLAGS= $(SDLFLAGS) -I ./ -I src/snd $(EXTFLAGS)
CFLAGS += -O3 -Wall ${INCLUDEFLAGS} -s -D USESDLMUTEXES -D STEREOOUTPUT -D LOWRESWAVETABLE# -D NOSDL_MIXER

chiptune_SRC = $(notdir ${wildcard *.c})
chiptune_DEP = $(patsubst %.c, %.d, ${chiptune_SRC})
chiptune_OBJ = $(patsubst %.c, objs/%.o, ${chiptune_SRC})

snd_SRC = $(notdir ${wildcard snd/*.c})
snd_DEP = $(patsubst %.c, snd_%.d, ${snd_SRC})
snd_OBJ = $(patsubst %.c, objs/snd_%.o, ${snd_SRC})

all: libchiptune.a sample

libchiptune.a: ${chiptune_OBJ} ${snd_OBJ}
	@$(ECHO) "Linking libchiptune..."
	@$(AR) $@ $^
	@$(RANLIB) $@

objs/%.o: %.c 
	@$(ECHO) "Compiling "$(notdir $<)"..."
	@$(MKDIR) -p objs
	$(CC) -c $(CFLAGS) -o $@ $<

objs/snd_%.o: snd/%.c 
	@$(ECHO) "Compiling "$(notdir $<)"..."
	@$(MKDIR) -p objs
	$(CC) -c $(CFLAGS) -o $@ $<

sample: libchiptune.a
	$(CC) -c $(CFLAGS) samples/sample.c -o samples/sample.o -I ./
	$(CC) -o samples/sample samples/sample.o libchiptune.a -lSDL2 -lSDL2_Mixer

clean:
	rm -fr libchiptune.a objs/ samples/sample.o samples/sample
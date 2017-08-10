CC= gcc
CFLAGS= -O2 -Wall -Wextra -std=gnu99

ECHO = echo
MKDIR= mkdir
AR= ar rcu
RANLIB= ranlib
RM= rm -f

SDLFLAGS = `sdl2-config --cflags` -U_FORTIFY_SOURCE
INCLUDEFLAGS= $(SDLFLAGS) -I ./
CFLAGS += -O3 -Wall ${INCLUDEFLAGS} -s -D USESDLMUTEXES -D STEREOOUTPUT -D LOWRESWAVETABLE

all: libchiptune.a

libchiptune.a: chiptune.o cyd.o cydchr.o cydcrush.o cydentry.o cydflt.o cydfm.o cydfx.o cydosc.o cydrvb.o cydwave.o freqs.o music.o pack.o
	@$(ECHO) "Linking libchiptune..."
	@$(AR) $@ $^
	@$(RANLIB) $@

%.o: %.c 
	@$(ECHO) "Compiling "$(notdir $<)"..."
	@$(MKDIR) -p objs
	$(CC) -c $(CFLAGS) -o $@ $<

sample: libchiptune.a
	$(CC) -c $(CFLAGS) samples/sample.c -o samples/sample.o -I ./
	$(CC) -o samples/sample samples/sample.o libchiptune.a -lSDL2 -lSDL2_Mixer

clean:
	rm -fr libchiptune.a *.o samples/sample.o samples/sample
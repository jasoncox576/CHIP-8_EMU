# CHIP-8_EMU
A fully-functional [CHIP-8 Emulator](https://en.wikipedia.org/wiki/CHIP-8) written in Rust. Final project for CS429H, Spring 2021 semester.

A CHIP-8 Emulator is widely considered to be a good intro project for those interested in making emulators for more complex retro game consoles like the Gameboy or NES. The CHIP-8 language itself was invented in the late 1970s in order to make it easy to write cross-platform games and programs for the various hobbyist computers of the day. A very old virtual ISA with simple specification, CHIP-8 escapes many of the complexities that emerge when working with real hardware. 

Technical Specification of the virtual ISA:

* 4096 bytes of RAM (the first 512 of which are historically occupied by the interpreter itself)
* 16 byte-size data registers
* A stack for return addresses
* Timers for cycle delay and sound
* Hexadecimal keypad to play games
* 35 word-size opcodes stored big-endian
* A monochrome 64x32 pixel display

## Some Details of Implementation ##

* Emulation is very simple, just a standard Fetch/Decode/Execute/Writeback loop, but Decode/Execute/Writeback can be treated as the same stage.
* Decoding is done via a giant switch statement, after which the appropriate state updates are made.
* Appropriate cycle time varies between games (some are made to run fasters, other slower), so I made it adjustable in-game using L-Shift/Tab keys.
* To generate the beeping sound, a standard 250hz sine wave tone was used.
* [ggez](https://github.com/ggez/ggez) used for the event handling, sound, graphic display, etc.


## How to Run ##

Cross-platform executable coming soon!


## Screenshots ##

Here are some screenshots from my implementation:

![alt text](https://github.com/jasoncox576/CHIP-8_EMU/blob/master/screenshots/Brix.png)
![alt text](https://github.com/jasoncox576/CHIP-8_EMU/blob/master/screenshots/sierpinksi.png)
![alt text](https://github.com/jasoncox576/CHIP-8_EMU/blob/master/screenshots/pong.png)
![alt text](https://github.com/jasoncox576/CHIP-8_EMU/blob/master/screenshots/space_invaders_title.png)
![alt text](https://github.com/jasoncox576/CHIP-8_EMU/blob/master/screenshots/space_invaders.png)



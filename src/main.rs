/*
NOTES
allocate 4kb memory
registers 0-F
Font
display - 64x32 grid of booleans
stack - 16-bit numbers
timers - decremented at 60hz
    delay
    sound
read keypresses - use scancodes


main loop
    fetch instruction
    decode
    execute
this loop may need to run at different speeds
default - 700 loops per second

fetch
    read two bytes from memory and combine into a 16-bit instruction
    increment PC by 2

decode
    extract all nibbles from instruction
    match on first half-byte

*/

use minifb::{Key, Window, WindowOptions};
use std::mem;

const WIDTH = 64;
const HEIGHT = 32;

fn main() {
    //set up registers, stack, timers

    let pc: u12 = 0;
    let index: u12 = 0;
    let (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF): u8 = 0;

    let mut stack: u16 = Vec::new();

    let mut sound_timer: u32 = 0;
    let mut delay_timer: u32 = 0;

    //allocate memory

    let mut vec: [u8; 4096] = [0; 4096];
    let mut mem = &mut vec[..];

    //initialize display

    let mut window = Window::new(
        "CHIP-8",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
            panic!("{}", e);
    });
    
    
}

fn fetch() -> void {
    
}

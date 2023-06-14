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

use minifb::{Key, Window, WindowOptions, ScaleMode};

const WIDTH: usize = 320; //chip-8 was 64x32
const HEIGHT: usize = 160;

fn main() {
    //set up registers, stack, timers

    let pc: u16 = 0;
    let index: u16 = 0;
    //let (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF): u8 = 0;

    let mut stack: Vec<u16> = Vec::new();

    let mut sound_timer: u32 = 0;
    let mut delay_timer: u32 = 0;

    //allocate memory

    let mut vec: [u8; 4096] = [0; 4096];
    let mut mem = &mut vec[..];

    //initialize display

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "CHIP-8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to creat window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while true {
        
        //update display
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
    
    
}

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

const PIXELW: i32 = 20;
const PIXELH: i32 = 20;

fn main() {
    //set up registers, stack, timers

    let pc: usize = 0;
    let index: u16 = 0;

    let V0: u8 = 0;
    let V1: u8 = 0;
    let V2: u8 = 0;
    let V3: u8 = 0;
    let V4: u8 = 0;
    let V5: u8 = 0;
    let V6: u8 = 0;
    let V7: u8 = 0;
    let V8: u8 = 0;
    let V9: u8 = 0;
    let VA: u8 = 0; 
    let VB: u8 = 0;
    let VC: u8 = 0;
    let VD: u8 = 0;
    let VE: u8 = 0;
    let VF: u8 = 0;

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

    //main loop
    loop {
        
        //update display
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        //fetch
        let b1: u8 = mem[pc];
        let b2: u8 = mem[pc + 1];
        pc += 2;

        //decode
        let n1: u8 = b1 >> 2;
        let n2: u8 = b1 << 6;
        n2 = n2 >> 6;
        let n3: u8 = b2 >> 2;
        let n4: u8 = b2 << 6;
        n4 = n4 >> 6;

        let instr = (n1, n2, n3, n4);

        //execute

        match instr {

            //clear screen
            (0,0,0xE,0) => buffer = vec![0 ; WIDTH * HEIGHT],
            (1, _, _, _) => pc = conv_to_addr(&instr),
            
            _ => panic!(),
        }

        
        
    }
    
    
}

//pc has to be a usize
fn conv_to_addr(instr: &(u8, u8, u8, u8)) -> usize {
    let addr: usize = 0;
    addr | instr.1;
    addr << 2;
    addr | instr.2;
    addr << 2;
    addr | instr.3;
    return addr;
}

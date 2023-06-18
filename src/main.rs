use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::Rng;
use std::env;
use std::fs;

const WIDTH: usize = 64; //chip-8 was 64x32
const HEIGHT: usize = 32;

const PIXELW: usize = 20;
const PIXELH: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bytes: Vec<u8> = fs::read(&args[1]).expect("Failed to read file.");

    //set up registers, stack, timers

    let mut rng = rand::thread_rng();

    let mut pc: usize = 0;
    let mut index: u16 = 0;

    //register array
    let mut regs: [u8; 16] = [0; 16];

    let mut stack: Vec<u16> = Vec::new();

    let mut sound_timer: u32 = 0;
    let mut delay_timer: u32 = 0;

    //allocate memory

    let mut vec: [u8; 4096] = [0; 4096];
    let mut mem = &mut vec[..];

    //read program into memory
    let mut i = 512;
    for byte in bytes {
        mem[i] = byte;
        i += 1;
    }

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

    let mut b1: u8;
    let mut b2: u8;
    let mut n1: u8;
    let mut n2: u8;
    let mut n3: u8;
    let mut n4: u8;

    let mut instr: (u8, u8, u8, u8);

    //main loop
    loop {
        //update display
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        //fetch
        b1 = mem[pc];
        b2 = mem[pc + 1];
        pc += 2;

        //decode
        n1 = b1 >> 2;
        n2 = b1 << 6;
        n2 = n2 >> 6;
        n3 = b2 >> 2;
        n4 = b2 << 6;
        n4 = n4 >> 6;

        instr = (n1, n2, n3, n4);

        //execute

        match instr {
            //clear screen
            (0, 0, 0xE, 0) => buffer = vec![0; WIDTH * HEIGHT],
            //jump
            (1, _, _, _) => pc = conv_to_addr(n2, n3, n4),
            //call
            (2, _, _, _) => {
                stack.push(pc as u16);
                pc = conv_to_addr(n2, n3, n4);
            }
            //return
            (0, 0, 0xE, 0xE) => pc = stack.pop().unwrap() as usize,
            //skip one instruction
            (3, _, _, _) => {
                let nn = n3 | n4;
                if nn == regs[n2 as usize] {
                    pc += 2;
                }
            }
            //skip
            (4, _, _, _) => {
                let nn = n3 | n4;
                if nn != regs[n2 as usize] {
                    pc += 2;
                }
            }
            (5, _, _, 0) => {
                if regs[n2 as usize] == regs[n3 as usize] {
                    pc += 2;
                }
            }
            (9, _, _, 0) => {
                if regs[n2 as usize] != regs[n3 as usize] {
                    pc += 2;
                }
            }
            //set
            (6, _, _, _) => {
                let nn = n3 | n4;
                regs[n2 as usize] = nn;
            }
            //Add
            (7, _, _, _) => {
                let nn = n3 | n4;
                regs[n2 as usize] += nn;
            }
            //Set register x to value of register y
            (8, _, _, 0) => {
                regs[n2 as usize] = regs[n3 as usize];
            }
            //OR
            (8, _, _, 1) => {
                regs[n2 as usize] = regs[n2 as usize] | regs[n3 as usize];
            }
            //AND
            (8, _, _, 2) => {
                regs[n2 as usize] = regs[n2 as usize] & regs[n3 as usize];
            }
            //XOR
            (8, _, _, 3) => {
                regs[n2 as usize] = regs[n2 as usize] ^ regs[n3 as usize];
            }
            //add reg to reg
            (8, _, _, 4) => {
                let res = regs[n2 as usize].overflowing_add(regs[n3 as usize]);
                if res.1 {
                    regs[0xF] = 1u8;
                } else {
                    regs[0xF] = 0u8;
                }
                regs[n2 as usize] = res.0;
            }
            //subtraction
            (8, _, _, 5) => {
                if n2 > n3 {
                    regs[0xF] = 1;
                } else {
                    regs[0xF] = 0;
                }
                regs[n2 as usize] = regs[n2 as usize].overflowing_sub(regs[n3 as usize]).0;
            }
            (8, _, _, 7) => {
                if n2 > n3 {
                    regs[0xF] = 1;
                } else {
                    regs[0xF] = 0;
                }
                regs[n2 as usize] = regs[n3 as usize].overflowing_sub(regs[n2 as usize]).0;
            }
            //right shift
            (8, _, _, 6) => {
                let mut digit = regs[n2 as usize] << 7;
                digit = digit >> 7;
                if digit == 1 {
                    regs[0xF] = 1;
                } else {
                    regs[0xF] = 0;
                }
                regs[n2 as usize] = regs[n2 as usize] >> 1;
            }
            //left shift
            (8, _, _, 0xE) => {
                if (regs[n2 as usize] >> 7) == 1 {
                    regs[0xF] = 1;
                } else {
                    regs[0xF] = 0;
                }
                regs[n2 as usize] = regs[n2 as usize] << 1;
            }
            //set index
            (0xA, _, _, _) => {
                let mut nnn: u16 = 0;
                nnn = nnn | n4 as u16;
                nnn = nnn | n3 as u16;
                nnn = nnn | n2 as u16;
                index = nnn;
            }
            //jump with offset
            (0xB, _, _, _) => {
                let mut nnn: u16 = 0;
                nnn = nnn | n4 as u16;
                nnn = nnn | n3 as u16;
                nnn = nnn | n2 as u16;
                nnn += regs[0] as u16;
                pc = nnn as usize;
            }
            //random
            (0xC, _, _, _) => {
                let nn = n3 | n4;
                let mut x: u8 = rng.gen_range(0..256) as u8;
                x = x & nn;
                regs[n2 as usize] = x;
            }
            //display
            (0xD, _, _, _) => {
                let x = regs[n2 as usize] & 63;
                let y = regs[n3 as usize] & 31;
                regs[0xF] = 0;

                for n in 0..regs[n4 as usize] {}
            }

            _ => panic!("Unknown Instruction."),
        }
    }
}

//pc has to be a usize
fn conv_to_addr(n2: u8, n3: u8, n4: u8) -> usize {
    let mut addr: usize = 0;

    let a = n2 as usize;
    let b = n3 as usize;
    let c = n4 as usize;

    addr = addr | a;
    addr = addr << 2;
    addr = addr | b;
    addr = addr << 2;
    addr = addr | c;
    return addr;
}

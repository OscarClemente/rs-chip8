extern crate rand;
extern crate sdl2;
mod drivers;
mod cpu;
mod fonts;

use std::env;
use std::thread;
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;

use sdl2::pixels;

use drivers::DisplayDriver;
use cpu::CPU;

fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();

    let mut cpu = CPU::new();


    let mut f = File::open("roms/test1.ch8").expect("File not found");
    let mut rom = [0u8; 3584];
    f.read(&mut rom).unwrap();
    cpu.load(&rom);

    let mut display_driver = DisplayDriver::new(
        &sdl_context,
        20,
        pixels::Color::RGB(0, 0, 0),
        pixels::Color::RGB(200, 200, 200),
    );

    let mut keypad = [false; 16];
    for _ in 0..1000 {
        let output = cpu.cycle(keypad);

        if output.vram_changed {
            println!("hello");
            println!("vram: {:?}", output.vram);
            display_driver.draw(&output.vram);
        }
        thread::sleep(Duration::from_millis(10));
    }

    /*thread::sleep(Duration::from_millis(1000));

    let mut vram = [[0u8; 64]; 32];

    for y in 0..32 {
        for x in 0..64 {
            if x % 2 == 0 {
                vram[y][x] = 1;
            } else {
                vram[y][x] = 0;
            }
        }
    }

    display_driver.draw(&vram);

    thread::sleep(Duration::from_millis(500));

    for i in 0..5 {
        for y in 0..32 {
            for x in 0..64 {
                if vram[y][x] == 0 {
                    vram[y][x] = 1;
                } else {
                    vram[y][x] = 0;
                }
            }
        }

        display_driver.draw(&vram);

        thread::sleep(Duration::from_millis(500));
    }*/
}

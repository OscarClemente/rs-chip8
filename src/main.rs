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
use drivers::KeypadDriver;
use drivers::AudioDriver;

use cpu::CPU;

fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(
        &sdl_context,
        20,
        pixels::Color::RGB(0, 0, 0),
        pixels::Color::RGB(200, 200, 200),
    );

    let mut cpu = CPU::new();
    let mut f = File::open("roms/pong.ch8").expect("File not found");
    let mut rom = [0u8; 3584];
    f.read(&mut rom).unwrap();
    cpu.load(&rom);

    let audio_driver = AudioDriver::new(&sdl_context);
    let mut keypad_driver = KeypadDriver::new(&sdl_context);

    while let Ok(keypad) = keypad_driver.poll() {
        let output = cpu.cycle(keypad);

        if output.vram_changed {
            display_driver.draw(&output.vram);
        }

        if output.beep {
            audio_driver.start_beep();
        } else {
            audio_driver.stop_beep();
        }

        thread::sleep(Duration::from_millis(5));
    }
}

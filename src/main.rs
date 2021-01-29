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
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(
        &sdl_context,
        config.background_color,
        config.foreground_color,
    );

    let mut cpu = CPU::new();
    let mut f = File::open(config.filepath).expect("File not found");
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

        thread::sleep(Duration::from_millis(config.frame_delay));
    }
}

pub struct Config {
    pub filepath: String,
    pub frame_delay: u64,
    pub foreground_color: sdl2::pixels::Color,
    pub background_color: sdl2::pixels::Color,
}

fn parse_args(args: &[String]) -> Config{
    if args.len() < 3
    {
        print_help();
        panic!("Wrong number of arguments.");
    }

    let filepath = args[1].clone();
    let frame_delay = args[2].parse::<u64>().unwrap();
    let mut foreground_color = pixels::Color::RGB(200, 200, 200);
    let mut background_color = pixels::Color::RGB(0, 0, 0);

    if args.len() >= 6 {
        foreground_color = pixels::Color::RGB(
            args[3].parse::<u8>().unwrap(),
            args[4].parse::<u8>().unwrap(),
            args[5].parse::<u8>().unwrap());
    }
    if args.len() >= 9 {
        background_color = pixels::Color::RGB(
            args[6].parse::<u8>().unwrap(),
            args[7].parse::<u8>().unwrap(),
            args[8].parse::<u8>().unwrap());
    }

    Config {
        filepath,
        frame_delay,
        foreground_color,
        background_color,
    }
}

fn print_help() {
    println!("Usage: rs-chip8 FILEPATH_TO_ROM FRAME_DELAY [FOREGROUND_COLOR] [BACKGROUND_COLOR]");
    println!("Emulates the rom in FILEPATH_TO_ROM with a delay between frames in miliseconds of FRAME_DELAY");
    println!("");
    println!("Colors are encoded as three RGB numbers with no delimitation.");
    println!("Example: rs-chip8 ./roms/pong.ch8 5 255 255 255 0 0 0");
    println!("");
}

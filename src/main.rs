extern crate rand;
extern crate sdl2;
mod drivers;

use std::env;
use std::thread;
use std::time::Duration;

use sdl2::pixels;

use drivers::DisplayDriver;

fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();

    let mut display_driver = DisplayDriver::new(
        &sdl_context,
        20,
        pixels::Color::RGB(0, 0, 0),
        pixels::Color::RGB(200, 200, 200),
    );

    thread::sleep(Duration::from_millis(1000));

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

    thread::sleep(Duration::from_millis(1000));

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

        thread::sleep(Duration::from_millis(1000));
    }
}

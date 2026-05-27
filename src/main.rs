use macroquad::prelude::*;
mod chip8;
use chip8::Emulator;

#[macroquad::main("CHIP8 Emulator")]
async fn main() {
    let em = Emulator::new();

    loop {
        let delta = get_frame_time();

        if delta < 1.0 / 60.0 {}
        next_frame().await
    }

    println!("Hello, world!");
}

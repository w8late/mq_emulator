mod chip8;
mod window;

use std::env;
use macroquad::prelude::*;
use chip8::{Emulator, rom};
use window::configuration;

#[macroquad::main(configuration)]
async fn main() {
    let mut em = Emulator::new();
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Pass a ROM file to the emulator!");
        return;
    }

    em.load_rom(rom::ROMLoader::new(&*args[1]).expect("can't open file"));

    loop {
        clear_background(BLACK);

        em.fde();
        em.display();
        draw_fps();
       
        next_frame().await
    }
}

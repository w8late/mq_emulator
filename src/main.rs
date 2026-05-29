mod chip8;
mod window;

use macroquad::prelude::*;
use chip8::{Emulator, rom};
use window::configuration;

#[macroquad::main(configuration)]
async fn main() {
    let mut em = Emulator::new();
    em.load_rom(rom::ROMLoader::new("roms/ibm_logo.ch8").expect("can't open file"));

    loop {
        clear_background(BLACK);

        em.fde();
        em.display();
        draw_fps();
       
        next_frame().await
    }
}

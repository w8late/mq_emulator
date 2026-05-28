use macroquad::prelude::*;

pub fn configuration() -> Conf {
    Conf {
        window_title: "CHIP8 Emulator".to_owned(),
        window_width: 640,
        window_height: 320,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}


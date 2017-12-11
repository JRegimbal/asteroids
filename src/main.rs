extern crate sfml;
pub mod entities;

use sfml::window::{Window, Event, VideoMode, Style, Key};
use sfml::graphics::{Texture};
use entities::ship::{Ship, Direction};
use std::borrow::Borrow;

fn main() {
    let width: u32 = 800;
    let height: u32 = 600;
    let desktop = VideoMode::desktop_mode();
    let mut window = Window::new(
                        VideoMode::new(width, height, desktop.bits_per_pixel),
                        "Asteroids",
                        Style::CLOSE,
                        &Default::default());
    window.set_framerate_limit(60);
    let texture: Texture = Texture::from_file(entities::ship::TEXTURE_LOC)
        .unwrap();
    
    // set up ship
    let mut ship = Ship::new(texture.borrow());
    ship.set_position((width/2 as f32, height/2 as f32));
    ship.set_rotation(0);
    ship.set_origin();


    while window.is_open() {
        for event in window.poll_event() {
            match event {
                Event::Closed       => {
                    window.close();
                },
                
                Event::KeyPressed {
                    code: Key::Space,
                    ..
                }                   => {
                    ship.fire();
                },

                _                   => {},

            }
        }


        window.display();
    }
}

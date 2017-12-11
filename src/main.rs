extern crate sfml;
pub mod entities;

use sfml::window::{Event, VideoMode, Style, Key};
use sfml::graphics::{Texture, RenderWindow, RenderTarget, Color};
use entities::ship::{Ship, Direction};
use entities::asteroid::Asteroid;
use std::borrow::Borrow;

fn main() {
    let width: u32 = 800;
    let height: u32 = 600;
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
                        VideoMode::new(width, height, desktop.bits_per_pixel),
                        "Asteroids",
                        Style::CLOSE,
                        &Default::default());
    window.set_framerate_limit(60);
    let texture: Texture = Texture::from_file(entities::ship::TEXTURE_LOC)
        .unwrap();
    
    // set up ship
    let mut ship = Ship::new(texture.borrow());
    ship.set_position((width as f32/2., height as f32/2.));
    ship.set_orientation(0.);
    ship.set_origin();

    let mut asteroid_vec: std::vec::Vec<Asteroid> = vec![];

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
        
        if window.has_focus() {
            if Key::Up.is_pressed() {
                println!("wat");
                ship.move_ship();
            }
            if Key::Left.is_pressed() || Key::Right.is_pressed() {
                ship.rotate_ship(
                    if Key::Left.is_pressed() {Direction::CCW}
                    else {Direction::CW}
                    );
            }
        }
        //check ship bounds
        ship.wrap_bounds(width as f32,height as f32);

        window.clear(&Color::WHITE);
        window.draw(&ship);
        window.display();
    }
}

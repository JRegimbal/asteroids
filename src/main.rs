extern crate sfml;
extern crate rand;

pub mod entities;

use sfml::window::{Event, VideoMode, Style, Key};
use sfml::graphics::{Texture, RenderWindow, RenderTarget, Color};
use sfml::system::Vector2f;
use entities::ship::{Ship, Bullet, Direction};
use entities::asteroid::{Asteroid, AsteroidGenerator};
use std::time::{Instant, Duration};
use std::borrow::Borrow;
use std::vec::Vec;

const MAX_ASTEROIDS: usize = 10;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let bounds: Vector2f = Vector2f::new(WIDTH as f32, HEIGHT as f32);
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
                        VideoMode::new(WIDTH, HEIGHT, desktop.bits_per_pixel),
                        "Asteroids",
                        Style::CLOSE,
                        &Default::default());
    window.set_framerate_limit(60);
    let ship_texture = Texture::from_file(entities::ship::TEXTURE_LOC)
        .unwrap();
    let asteroid_texture = Texture::from_file(entities::asteroid::TEXTURE_LOC)
        .unwrap();
    
    // set up ship
    let mut ship = Ship::new(ship_texture.borrow());
    ship.set_position((bounds.x/2., bounds.y/2.));
    ship.set_orientation(0.);
    ship.set_origin();

    let mut last_hit: Instant = Instant::now();

    let mut asteroid_vec: Vec<Asteroid> = vec![];
    let mut bullet_vec: Vec<Bullet> = vec![];
    
    let mut points: u8 = 0;
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
                    if ship.is_alive() {
                        if bullet_vec.len() < 4 {
                            let pos = ship.fire(); 
                            let b = Bullet::new(pos, ship.get_orientation());
                            bullet_vec.push(b);
                        }   
                    }
                },

                _                   => {},

            }
        }
        
        if ship.is_alive() {
            if window.has_focus() {
                if Key::Up.is_pressed() {
                    ship.move_ship();
                }
                if Key::Left.is_pressed() || Key::Right.is_pressed() {
                    ship.rotate_ship(
                        if Key::Left.is_pressed() {Direction::CCW}
                        else {Direction::CW}
                        );
                }
            }

            if asteroid_vec.len() < MAX_ASTEROIDS {
                let a = AsteroidGenerator::generate(&bounds, asteroid_texture.borrow());
                asteroid_vec.push(a);
            }
            //check ship bounds
            ship.wrap_bounds(bounds.x, bounds.y);

            bullet_vec.iter_mut().for_each(|b| b.update());
            asteroid_vec.iter_mut().for_each(|a| a.update());

            let mut temp: Vec<Asteroid> = vec![];
            for a in asteroid_vec.iter().filter_map(|a| a.split()) {
                for i in a.into_iter() {
                    temp.push(i);
                }
            }
            asteroid_vec.append(&mut temp);
            
            asteroid_vec.retain(|a| a.is_alive() && a.in_bounds());
            bullet_vec.retain(|b| b.in_bounds(bounds.x, bounds.y));
            
            //check collisions
            //bullet and asteroid
            for a in asteroid_vec.iter_mut() {
                for b in bullet_vec.iter_mut() {
                    if a.contains(b.position()) {
                        a.is_hit();
                        b.destroy();
                        points += 1;
                    }
                }
            }
            
            //asteroid and ship
            if last_hit.elapsed() >= Duration::new(2,0) {
                for a in asteroid_vec.iter_mut() {
                    if a.intersects(&ship.rect()) {
                        ship.take_damage();
                        last_hit = Instant::now();
                        a.destroy();
                        break;
                    }
                }
            }
        }

        window.clear(&Color::WHITE);
        window.draw(&ship);
        for b in &bullet_vec {
            window.draw(b);
        }
        for a in &asteroid_vec {
            window.draw(a);
        }
        window.display();
    }
    println!("Final Score: {}", points);
}

extern crate sfml;
extern crate std;

use sfml::graphics::{Texture, Sprite, Transformable};
use sfml::system::Vector2f;

const BREAK_NUM: u8 = 4;

enum AsteroidSize {
    Large,
    Medium,
    Small,
}

pub struct Asteroid<'a> {
    sprite: Sprite<'a>,
    size: AsteroidSize,
    direction: Vector2f,
    pts: u8,
}

impl<'a> Asteroid<'a> {
    pub fn new(initial_pos: Vector2f, direction: Vector2f, texture: &'a Texture) -> Asteroid<'a> {
        let sprite: Sprite = Sprite::with_texture(texture);
        let size: AsteroidSize = AsteroidSize::Large;
        let direction = direction;

        Asteroid {
            sprite: sprite,
            size: size,
            direction: direction,
            pts: 3,
        }
    }

    fn split(&self) -> Option<std::vec::Vec<Asteroid>> {
        match self.size {
            AsteroidSize::Small => {return None;}
            _                   => {
                let mut new_asteroids = vec![];
                for x in 0..BREAK_NUM {
                    let sprite = self.sprite.clone();
                    let size = match self.size {
                        AsteroidSize::Large     => AsteroidSize::Medium,
                        _                       => AsteroidSize::Small,
                    };
                    let pts = match size {
                        AsteroidSize::Medium    => 2,
                        _                       => 1,
                    };
                    let a = Asteroid {
                        sprite: sprite,
                        size: size,
                        direction: self.direction.clone(),
                        pts: pts,
                    };
                    new_asteroids.push(a);
                }
                return Some(new_asteroids);
            },
        }
    }       

    pub fn is_hit(&mut self) {
        if self.pts > 0 {
            self.pts -= 1;
        }
    }

    pub fn update(&mut self) -> Option<std::vec::Vec<Asteroid>> {
        if self.pts == 0 { 
            return self.split()
        }
        else {
            let x = self.sprite.position().x + self.direction.x;
            let y = self.sprite.position().y + self.direction.y;
            self.sprite.set_position(Vector2f::new(x, y));
        }
        None
    }
}

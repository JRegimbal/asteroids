extern crate sfml;
extern crate std;

use sfml::graphics::{Texture, Sprite, Transformable};
use sfml::system::{Vector2f};

const INCREMENT: u32 = 5;
pub const TEXTURE_LOC: &str = "ship.png";

pub enum Direction {
    North = 0b00000001,
    West = 0b00000010,
    East = 0b00000100,
    South = 0b00001000,
}

pub struct Ship<'a> {
    sprite: Sprite<'a>,
    life: u8,
    shots: u8,
    orientation: u8,
}

impl<'a> Ship<'a> {
    pub fn new(texture: &'a Texture) -> Ship<'a> {
        let sprite: Sprite = Sprite::with_texture(texture);
        let orientation = Direction::North as u8;
        Ship { sprite: sprite, life: 3, shots: 0, orientation: orientation }
    }

    pub fn move_ship(&mut self, direction: Direction) {

    }

    pub fn fire(&mut self) {

    }

    pub fn take_damage(&mut self) {
        if self.life > 0 {
            self.life -= 1;
        }
    }

    pub fn is_alive(&self) -> bool {
        (self.life > 0)
    }

    pub fn get_position(&self) -> Vector2f {
        self.sprite.position()
    }

    pub fn set_position<P: Into<Vector2f>>(&mut self, position: P) -> Vector2f {
        self.sprite.set_position(position);
        self.sprite.position()
    }

    pub fn get_orientation(&self) -> u8 {
        self.orientation
    }

    pub fn set_orientation(&mut self, angle: f32) {
        self.sprite.set_rotation(angle);
    }

    pub fn set_origin(&mut self) {
        let width = self.sprite.local_bounds().width / 2.;
        let height = self.sprite.local_bounds().height / 2.;
        self.sprite.set_origin((width, height));
    }
}

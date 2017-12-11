extern crate sfml;
extern crate std;

use sfml::graphics::{Texture, Sprite, Transformable, Drawable, RenderStates, RenderTarget};
use sfml::system::{Vector2f};

pub const TEXTURE_LOC: &str = "ship.png";
const MOVE_INC: f32 = 6.;
const ROTATE_INC: f32 = 3.;
const SHOT_LIMIT: u8 = 4;

pub enum Direction {
   CW,
   CCW,
}

pub struct Ship<'a> {
    sprite: Sprite<'a>,
    life: u8,
    shots: u8,
}

impl<'a> Ship<'a> {
    pub fn new(texture: &'a Texture) -> Ship<'a> {
        let sprite: Sprite = Sprite::with_texture(texture);
        Ship { sprite: sprite, life: 3, shots: 0 }
    }

    pub fn move_ship(&mut self) {
        let theta = self.sprite.rotation()*std::f32::consts::PI/180.;
        let old_x = self.sprite.position().x;
        let old_y = self.sprite.position().y;
        let new_x = old_x + theta.sin()*MOVE_INC;
        let new_y = old_y - theta.cos()*MOVE_INC;
        self.sprite.set_position((new_x, new_y));
    }

    pub fn rotate_ship(&mut self, direction: Direction) {
        let theta = self.sprite.rotation() + match direction {
            Direction::CW   => ROTATE_INC,
            _               => -ROTATE_INC,
        };
        self.sprite.set_rotation(theta);
    }

    pub fn wrap_bounds(&mut self, width: f32, height: f32) {
        let orig_x = self.sprite.global_bounds().left + self.sprite.global_bounds().width/2.;
        let orig_y = self.sprite.global_bounds().top - self.sprite.global_bounds().height/2.;
        let origin = Vector2f::new(orig_x, orig_y);
        let mut x = self.sprite.position().x;
        let mut y = self.sprite.position().y;
        if origin.x <= 0. {
            x += width;
        } else if origin.x >= width {
            x -= width;
        }
        if origin.y <= 0. {
            y += height;
        } else if origin.y >= height {
            y -= height;
        }

        self.sprite.set_position((x,y));
    }

    pub fn fire(&mut self) -> std::option::Option<Vector2f> {
        if self.shots < SHOT_LIMIT {
            self.shots += 1;
            return Some(self.get_position());
        } else {
            return None;
        }
    }

    pub fn reclaim_shot(&mut self) {
        self.shots -= 1;
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

    pub fn get_orientation(&self) -> f32 {
        self.sprite.rotation()
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

impl<'a> Drawable for Ship<'a> {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
        ) {
        render_target.draw(&self.sprite);
    }
}

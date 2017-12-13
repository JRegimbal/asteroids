extern crate std;

use sfml::graphics::{Texture, Sprite, Transformable, Drawable, RenderTarget, RenderStates, FloatRect};
use sfml::system::Vector2f;
use rand::{thread_rng, Rng};

pub const TEXTURE_LOC: &str = "res/asteroid.png";
const BREAK_NUM: u8 = 4;
const MOVE_INC: f32 = 3.;

#[derive(Clone)]
enum AsteroidSize {
    Large,
    Medium,
    Small,
}

pub struct Asteroid<'a> {
    sprite: Sprite<'a>,
    size: AsteroidSize,
    theta: f32,
    pts: u8,
    has_entered: bool,
    bounds: &'a Vector2f,
    speed: f32,
}

impl<'a> Asteroid<'a> {
    pub fn new(initial_pos: Vector2f, theta: f32, texture: &'a Texture, bounds: &'a Vector2f) -> Asteroid<'a> {
        let mut sprite: Sprite = Sprite::with_texture(texture);
        sprite.set_position(initial_pos);
        let size: AsteroidSize = AsteroidSize::Large;
        let theta = theta;
        let mut rng = thread_rng();
        let speed = MOVE_INC + rng.gen_range(-1.5, 1.);

        Asteroid {
            sprite: sprite,
            size: size,
            theta: theta,
            pts: 3,
            has_entered: false,
            bounds: bounds,
            speed: speed,
        }
    }

    pub fn split(&self) -> Option<std::vec::Vec<Asteroid<'a>>> {
        if self.pts > 0 {
            return None;
        }
        match self.size {
            AsteroidSize::Small => {return None;}
            _                   => {
                let mut new_asteroids = vec![];
                for x in 0..BREAK_NUM {
                    let mut sprite = self.sprite.clone();
                    let size = match self.size {
                        AsteroidSize::Large     => AsteroidSize::Medium,
                        _                       => AsteroidSize::Small,
                    };
                    let pts = match size {
                        AsteroidSize::Medium    => 2,
                        _                       => 1,
                    };
                    if pts == 2 {
                        sprite.set_scale((0.5,0.5));
                    } else {
                        sprite.set_scale((0.25,0.25));
                    }
                    let a = Asteroid {
                        sprite: sprite.clone(),
                        size: size.clone(),
                        theta: self.theta + (x*BREAK_NUM) as f32/360.,
                        pts: pts,
                        has_entered: true,
                        bounds: self.bounds,
                        speed: self.speed,
                    };
                    new_asteroids.push(a);
                }
                return Some(new_asteroids);
            },
        }
    }       

    pub fn contains(&self, pos: Vector2f) -> bool {
        self.sprite.global_bounds().contains(pos)
    }

    pub fn intersects(&self, rect: &FloatRect) -> bool {
        match self.sprite.global_bounds().intersection(rect) {
            Some(_) => true,
            _       => false,
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.bounded() || !self.has_entered
        
    }

    pub fn is_alive(&self) -> bool {
        self.pts > 0
    }

    pub fn is_hit(&mut self) {
        if self.pts > 0 {
            self.pts -= 1;
        }
    }

    pub fn destroy(&mut self) {
        self.sprite.set_position((-100.0, -100.0));
    }

    pub fn update(&mut self) {
        if self.pts > 0 { 
            let theta = self.theta*std::f32::consts::PI/180.;
            let x = self.sprite.position().x + theta.sin()*MOVE_INC;
            let y = self.sprite.position().y - theta.cos()*MOVE_INC;
            self.sprite.set_position((x, y));
            if self.bounded() { self.has_entered = true; }
        }
    }

    fn bounded(&self) -> bool {
        let x = self.sprite.position().x;
        let y = self.sprite.position().y;
        x >= -self.sprite.global_bounds().width && x <= self.bounds.x && y >= -self.sprite.global_bounds().height && y <= self.bounds.y
    }
}

impl<'a> Drawable for Asteroid<'a> {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
        ) {
        render_target.draw(&self.sprite);
    }
}

impl<'a> Clone for Asteroid<'a> {
    fn clone(&self) -> Asteroid<'a> { 
        Asteroid {
            sprite: self.sprite.clone(),
            size: self.size.clone(),
            theta: self.theta,
            pts: self.pts,
            has_entered: self.has_entered,
            bounds: self.bounds,
            speed: self.speed,
        }
    }
}

pub struct AsteroidGenerator {
}
impl AsteroidGenerator {
    pub fn generate<'a>(bounds: &'a Vector2f, texture: &'a Texture) -> Asteroid<'a> {
        let mut rng = thread_rng();
        let theta: f32 = rng.gen_range(0., 360.) * std::f32::consts::PI / 180.;
        let origin = Vector2f::new(bounds.x/2., bounds.y/2.);
        let dist: f32 = (origin.x.powf(2.) + origin.y.powf(2.)).sqrt() * 1.25;
        let x = origin.x + theta.sin()*dist;
        let y = origin.y - theta.cos()*dist;
        let angle_deviation: f32 = rng.gen_range(-10., 10.);
        Asteroid::new(Vector2f::new(x,y), theta*180./std::f32::consts::PI + 180. + angle_deviation, texture, bounds)
    }
}

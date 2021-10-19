use super::{
    raylib::prelude::*,
    animation::Animation,
    misc_funcs::Transform
};

pub struct Enemy {
    pub sprite: Texture2D,
    pub animation: Animation,
    speed: f32,
    pub transform: Transform
}

impl Enemy {
    pub fn new(sprite: Texture2D, rows: f32, cols: f32,frame: f32, speed: f32, scale: f32) -> Enemy {

        let vec = Vector2::new(
            sprite.width as f32 / rows, 
            sprite.height as f32 / cols
        );

        Enemy {
            transform: Transform::new(
                Vector2::new(0.0, 0.0), 
                scale, 
                1.0
            ),
            sprite,
            animation: Animation::new(
                vec, 
                frame
            ), 
            speed
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}
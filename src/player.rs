use super::{
    raylib::prelude::*,
    animation::*
};


pub struct Player {
    pub sprite: Texture2D,
    pub animation: Animation,
    pub pos: Vector2,
    pub speed: f32
}

impl Player {
    pub fn new(sprite_texture: Texture2D, rows: f32, cols: f32, frame: f32, speed: f32) -> Player{
        let vec = Vector2::new(
            sprite_texture.width as f32 / rows,
            sprite_texture.height as f32 / cols
        );

        Player {
            sprite: sprite_texture,
            animation: Animation::new(
                vec,
                frame
            ),
            pos: Vector2::new(0.0,0.0),
            speed
        }
    }

    // TODO: Investigate Math Vector && Add Enum for the Player animations
    pub fn move_player(&mut self, vec: Vector2){
        self.pos += vec.normalized();

        self.animation.change_animation(8.0, 1.0);
    }
}
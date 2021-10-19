use super::{
    raylib::prelude::*
};

pub struct Transform {
    position: Vector2,
    scale: f32,
    direction: f32  
}

pub struct Sprites_Textures {
    current_sprite: Texture2D,
    idle_sprite: Texture2D,
    run_texture: Texture2D
}

impl Transform {
    pub fn new(position: Vector2, scale: f32, direction: f32) -> Transform {
        Transform {
            position,
            scale,
            direction
        }
    }

    pub fn new_blank() -> Transform {
        Transform {
            position: Vector2::new(0.0, 0.0),
            scale: 1.0,
            direction: 1.0
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_direction(&self) -> f32 {
        self.direction
    }

    pub fn set_position(&mut self, vec: Vector2) {
        self.position = vec;
    }

    pub fn set_scale(&mut self, scale: f32){
        self.scale = scale;
    }

    pub fn set_direction(&mut self, direction:f32){
        self.direction = direction;
    }

    pub fn add_to_position(&mut self, vec: Vector2){
        self.position += vec;
    }
}


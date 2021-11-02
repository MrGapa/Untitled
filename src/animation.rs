use std::collections::HashMap;

use super::{
    raylib::prelude::*
};

pub struct AnimationData {
    row: f32,
    total_frames: f32,
}

/**
 * General Struct for animation
 */
pub struct Animation {
    pub rec: Rectangle,
    pub total_frames: f32,
    current_frame: f32,
    animation_map: HashMap<&'static str, AnimationData>,
    current_animation_name: &'static str
}

impl Animation {
    /** 
     * Creates a new Aniamtion scturct
     */
    pub fn new(sprite_size: Vector2, total_frames: f32) -> Animation {
        Animation {
            rec: Rectangle::new(0.0, 0.0, sprite_size.x, sprite_size.y),
            total_frames,
            current_frame: 0.0,
            animation_map: HashMap::new(),
            current_animation_name: "IDLE"
        }
    }

    /**
     * Plays animation. Increments the X position of The Rectangle
     */
    pub fn play_animation(&mut self) {
        self.current_frame += 1.0;

        if self.current_frame > self.total_frames - 1.0 { self.current_frame = 0.0}
        self.rec.x = self.current_frame * self.rec.width;
    } 

    pub fn add_animation(&mut self, name: &'static str, row: f32, total_frames: f32){
        self.animation_map.insert(
            name, 
            AnimationData{
                row,
                total_frames
            }
        );
    }
    
    pub fn change_animation(&mut self, name: &'static str) {
        let anim_data = self.animation_map.get(name).unwrap();

        self.total_frames = anim_data.total_frames;
        self.rec.y = anim_data.row * self.rec.height;
    }
}
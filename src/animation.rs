use super::{
    raylib::prelude::*
};

/**
 * General Struct for animation
 */
pub struct Animation {
    pub rec: Rectangle,
    pub frame: f32,
    pub current_frame: f32,
}

impl Animation {
    /** 
     * Creates a new Aniamtion scturct
     */
    pub fn new(sprite_size: Vector2, frame: f32) -> Animation {
        Animation {
            rec: Rectangle::new(0.0, 0.0, sprite_size.x, sprite_size.y),
            frame,
            current_frame: 0.0,
        }
    }

    /**
     * Plays animation. Increments the X position of The Rectangle
     */
    pub fn play_animation(&mut self) {
        self.current_frame += 1.0;

        if self.current_frame > self.frame - 1.0 { self.current_frame = 0.0}
        self.rec.x = self.current_frame * self.rec.width;
    } 
    
    pub fn change_animation(&mut self, frame: f32, animation_idx: f32) {
        self.frame = frame;

        self.rec.y = animation_idx * self.rec.height;   
    }
}
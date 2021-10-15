use super::{
    raylib::prelude::*,
    animation::*
};

pub enum PlayerAnimations {
    IDLE,
    RUN,
    ATTACK,
    JUMP,
    JUMPTOFALL,
    FALL,
    DEATH,
    HURT
}

pub struct Player {
    pub sprite: Texture2D,
    pub animation: Animation,
    pub pos: Vector2,
    pub speed: f32,
    pub player_direction: f32
}

impl Player {
    pub fn new(sprite_texture: Texture2D, rows: f32, cols: f32, frame: f32, speed: f32) -> Player{
        let vec = Vector2::new(
            sprite_texture.width as f32 / rows,
            sprite_texture.height as f32 / cols
        );

        let mut p = Player {
            sprite: sprite_texture,
            animation: Animation::new(
                vec,
                frame
            ),
            pos: Vector2::new(0.0,0.0),
            speed,
            player_direction: 1.0
        };

        p.update_animation(PlayerAnimations::IDLE);

        p
    }

    // TODO: Investigate Math Vector && Add Enum for the Player animations
    pub fn move_player(&mut self, vec: Vector2){
        self.pos += vec;
    }

    pub fn update_animation(&mut self, p_anim: PlayerAnimations){
        match p_anim {
            PlayerAnimations::IDLE => self.animation.change_animation(6.0, 0.0),
            PlayerAnimations::RUN => self.animation.change_animation(8.0, 1.0),
            _ => self.animation.change_animation(6.0, 0.0)
        }
    }
}

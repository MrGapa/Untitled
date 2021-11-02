use crate::player::Player;

use super::{
    raylib::prelude::*,
    enemy::Enemy
};

pub struct Scene {
    name: &'static str,
    pub background: Texture2D,
}

impl Scene {
    pub fn new(name: &'static str, background: Texture2D) -> Scene {
        Scene {
            name,
            background,
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, player: &mut Player) {
        self.render_background(d);
        
        d.draw_texture_pro(
            &player.sprite, 
            Rectangle::new(
                player.animation.rec.x,
                player.animation.rec.y, 
                player.transform.get_direction() * player.animation.rec.width, 
                player.animation.rec.height
            ), 
            Rectangle::new(
                player.transform.get_position().x, 
                player.transform.get_position().y,  
                player.transform.get_scale() * player.animation.rec.width, 
                player.transform.get_scale() * player.animation.rec.height
            ),
            Vector2::new(0.0,0.0), 
            0.0, 
            Color::WHITE
        );
    }

    fn render_background(&self,d: &mut RaylibDrawHandle){
        d.draw_texture_ex(
            &self.background, 
            Vector2::new(0.0, 0.0),
            0.0,
            2.0,
            Color::RAYWHITE
        );
    }
}
use std::collections::HashMap;

use raylib::{RaylibHandle, prelude::RaylibDrawHandle};

use super::{
    scene::Scene,
    player::Player,
    consts::*
};

pub struct GameManager {
    scenes: HashMap<&'static str, Scene>,
    current_scene: &'static str,
    animation_time: f32
}

impl GameManager {
    pub fn new() -> GameManager {
        GameManager { 
            scenes: HashMap::new(), 
            current_scene: "", 
            animation_time: 0.0 
        }
    }

    pub fn add_scene(&mut self, name: &'static str, scene: Scene) {
        self.scenes.insert(name,scene);
        self.current_scene = name;
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, player: &mut Player) {
        let scene = self.scenes.get(self.current_scene).unwrap();

        scene.render(d, player);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, player: &mut Player) {
        let dt = rl.get_frame_time();

        self.animation_time += dt;

        if self.animation_time >= UPDATE_TIME {
            self.animation_time = 0.0;

            player.animation.play_animation();
            //enemy.animation.play_animation();
        }
    }
}
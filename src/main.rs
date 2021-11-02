extern crate raylib;

use raylib::{
    prelude::*
};

mod consts;
mod animation;
mod input_handler;
mod enemy;
mod player;
mod misc_funcs;
mod character;
mod game_manager;
mod scene;

use consts::*;
use player::*;
use input_handler::*;
use enemy::*;
use scene::Scene;
use game_manager::GameManager;

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(W_WIDTH, W_HEIGHT)
    .title("Untitled")
    .msaa_4x()
    .build();
    
    let mut animation_time = 0.0;
    let mut player_speed: f32 = 0.0;
    let mut enemy_speed: f32 = 0.0;

    rl.set_exit_key(Some(KeyboardKey::KEY_F10));

    let mut manager = GameManager::new();

    let ugo = Scene::new(
        "INIT", 
        rl.load_texture(&thread, "assets/UGO.png").expect("TEXTURE NOT FOUND")
    );

    manager.add_scene(ugo.get_name(), ugo);
    
    let mut player = Player::new(
        rl.load_texture(&thread, "assets/Player/Character_Sheet.png").expect("Couldn't Load Player Texture"),
        12.0, 
        8.0,
        6.0,
        MOVEMENT_FORCE,
        2.0
    );

    let mut enemy = Enemy::new(
        rl.load_texture(&thread, "assets/Player/Idle.png").expect("Couln't load texture"), 
        8.0, 
        1.0, 
        8.0, 
        MOVEMENT_FORCE, 
        2.0
    );

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {

        manager.update(&mut rl, &mut player);

        // handle_player_inputs(&rl, &mut player, &mut player_speed, &dt);
        // handle_enemy_inputs(&mut enemy, &mut enemy_speed, &dt);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        manager.render(&mut d, &mut player);

        // d.draw_texture_pro(
        //     &enemy.sprite, 
        //     Rectangle::new(enemy.animation.rec.x, enemy.animation.rec.y, enemy.transform.get_direction() * enemy.animation.rec.width, enemy.animation.rec.height), 
        //     Rectangle::new(enemy.transform.get_position().x, enemy.transform.get_position().y, enemy.transform.get_scale() * enemy.animation.rec.width, enemy.transform.get_scale() * enemy.animation.rec.height), 
        //     Vector2::new(50.0,50.0),
        //     0.0, 
        //     Color::WHITE, 
        // );

        
    }
}

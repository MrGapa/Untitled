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

use consts::*;
use player::*;
use input_handler::*;
use enemy::*;

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(W_WIDTH, W_HEIGHT)
    .title("Untitled")
    .msaa_4x()
    .build();
    
    let mut animation_time = 0.0;
    let mut characters_speed: f32 = 0.0;

    rl.set_exit_key(Some(KeyboardKey::KEY_F10));
    
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

        let dt = rl.get_frame_time();

        animation_time += dt;

        if animation_time >= UPDATE_TIME {
            animation_time = 0.0;

            player.animation.play_animation();
            enemy.animation.play_animation();
        }

        handle_player_inputs(&rl, &mut player, &mut characters_speed, &dt);
        handle_enemy_inputs(&mut enemy, &mut characters_speed, &dt);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::VIOLET);

        d.draw_texture_pro(
            &player.sprite, 
            Rectangle::new(player.animation.rec.x,player.animation.rec.y, player.transform.get_direction() * player.animation.rec.width, player.animation.rec.height), 
            Rectangle::new(player.transform.get_position().x, player.transform.get_position().y,  player.transform.get_scale() * player.animation.rec.width, player.transform.get_scale() * player.animation.rec.height),
            Vector2::new(0.0,0.0), 
            0.0, 
            Color::WHITE
        );

        d.draw_texture_pro(
            &enemy.sprite, 
            Rectangle::new(enemy.animation.rec.x, enemy.animation.rec.y, enemy.transform.get_direction() * enemy.animation.rec.width, enemy.animation.rec.height), 
            Rectangle::new(enemy.transform.get_position().x, enemy.transform.get_position().y, enemy.transform.get_scale() * enemy.animation.rec.width, enemy.transform.get_scale() * enemy.animation.rec.height), 
            Vector2::new(50.0,50.0),
            0.0, 
            Color::WHITE, 
        );
    }
}

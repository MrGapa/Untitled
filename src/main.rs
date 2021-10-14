extern crate raylib;

use raylib::{
    prelude::*
};

mod consts;
mod player;
mod animation;
mod input_handler;

use consts::*;
use player::*;
use input_handler::*;

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
        MOVEMENT_FORCE * 100.0
    );

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {

        let dt = rl.get_frame_time();

        animation_time += dt;

        if animation_time >= UPDATE_TIME {
            animation_time = 0.0;

            player.animation.play_animation();
        }

        handle_player_inputs(&rl, &mut player, &mut characters_speed, &dt);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::VIOLET);
        d.draw_text("Hi", W_WIDTH / 2, W_HEIGHT / 2, 20, Color::BLACK);
        d.draw_texture_rec(&player.sprite, player.animation.rec, player.pos, Color::WHITE);
    }
}
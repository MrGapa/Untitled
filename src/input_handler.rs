use super::{
    raylib::prelude::*,
    player::Player
};

// TODO: Refine Player Movement 

pub fn handle_player_inputs(rl: &RaylibHandle, player: &mut Player, characters_speed: &mut f32, dt: &f32){
    if rl.is_key_down(KeyboardKey::KEY_D) {
        *characters_speed += player.speed;

        player.move_player(Vector2::new(*characters_speed * dt, 0.0));
    }

    if rl.is_key_down(KeyboardKey::KEY_A) {
        *characters_speed += player.speed;

        player.move_player(Vector2::new(-*characters_speed * dt, 0.0));
    }

    if rl.is_key_down(KeyboardKey::KEY_S) {
        *characters_speed += player.speed;

        player.move_player(Vector2::new(0.0, *characters_speed * dt));
    }

    if rl.is_key_down(KeyboardKey::KEY_W) {
        *characters_speed += player.speed;

        player.move_player(Vector2::new(0.0, -*characters_speed * dt));
    }
}
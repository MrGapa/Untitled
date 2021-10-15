use super::{
    raylib::prelude::*,
    player::{
        Player, PlayerAnimations
    }
};

pub fn handle_player_inputs(rl: &RaylibHandle, player: &mut Player, characters_speed: &mut f32, dt: &f32){
    if rl.is_key_down(KeyboardKey::KEY_D) {
        *characters_speed += 1.0;

        let player_movement = Vector2::new(*characters_speed * dt, 0.0);

        player.move_player(player_movement.scale_by(player.speed));
        player.update_animation(PlayerAnimations::RUN);
        player.player_direction = 1.0;
    } else if rl.is_key_released(KeyboardKey::KEY_D) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_down(KeyboardKey::KEY_A) {
        *characters_speed += 1.0;

        let player_movement = Vector2::new(-*characters_speed * dt, 0.0);

        player.move_player(player_movement.scale_by(player.speed));
        player.update_animation(PlayerAnimations::RUN);
        player.player_direction = -1.0;
    } else if rl.is_key_released(KeyboardKey::KEY_A) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_down(KeyboardKey::KEY_S) {
        *characters_speed += 1.0;

        let player_movement = Vector2::new(0.0, *characters_speed * dt);

        player.move_player(player_movement.scale_by(player.speed));
        player.update_animation(PlayerAnimations::RUN);
    } else if rl.is_key_released(KeyboardKey::KEY_S) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_down(KeyboardKey::KEY_W) {
        *characters_speed += 1.0;

        let player_movement = Vector2::new(0.0, -*characters_speed * dt);

        player.move_player(player_movement.scale_by(player.speed));
        player.update_animation(PlayerAnimations::RUN);
    } else if rl.is_key_released(KeyboardKey::KEY_W) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_up(KeyboardKey::KEY_A) || rl.is_key_up(KeyboardKey::KEY_W) || rl.is_key_up(KeyboardKey::KEY_S) || rl.is_key_up(KeyboardKey::KEY_D) {
        *characters_speed = 0.0;
    }
}
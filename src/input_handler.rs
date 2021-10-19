use super::{
    raylib::prelude::*,
    player::{
        Player, PlayerAnimations
    },
    enemy::Enemy,
    consts::W_WIDTH
};

pub fn handle_player_inputs(rl: &RaylibHandle, player: &mut Player, character_speed: &mut f32, dt: &f32){
    if rl.is_key_down(KeyboardKey::KEY_D) {
        *character_speed += 1.0;

        let player_movement = Vector2::new(*character_speed * dt, 0.0);

        player.move_player(player_movement.scale_by(player.get_speed()));
        player.update_animation(PlayerAnimations::RUN);
        player.transform.set_direction(1.0);
    } else if rl.is_key_released(KeyboardKey::KEY_D) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_down(KeyboardKey::KEY_A) {
        *character_speed += 1.0;

        let player_movement = Vector2::new(-*character_speed * dt, 0.0);

        player.move_player(player_movement.scale_by(player.get_speed()));
        player.update_animation(PlayerAnimations::RUN);
        player.transform.set_direction(-1.0);
    } else if rl.is_key_released(KeyboardKey::KEY_A) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_down(KeyboardKey::KEY_S) {
        *character_speed += 1.0;

        let player_movement = Vector2::new(0.0, *character_speed * dt);

        player.move_player(player_movement.scale_by(player.get_speed()));
        player.update_animation(PlayerAnimations::RUN);
    } else if rl.is_key_released(KeyboardKey::KEY_S) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_down(KeyboardKey::KEY_W) {
        *character_speed += 1.0;

        let player_movement = Vector2::new(0.0, -*character_speed * dt);

        player.move_player(player_movement.scale_by(player.get_speed()));
        player.update_animation(PlayerAnimations::RUN);
    } else if rl.is_key_released(KeyboardKey::KEY_W) {
        player.update_animation(PlayerAnimations::IDLE);
    }

    if rl.is_key_up(KeyboardKey::KEY_A) || rl.is_key_up(KeyboardKey::KEY_W) || rl.is_key_up(KeyboardKey::KEY_S) || rl.is_key_up(KeyboardKey::KEY_D) {
        *character_speed = 0.0;
    }
}

pub fn handle_enemy_inputs(enemy: &mut Enemy, character_speed: &mut f32, dt: &f32){
    *character_speed += 1.0;

    let movement = Vector2::new(*character_speed * dt, 0.0);

    enemy.transform.set_direction(1.0);
    enemy.transform.add_to_position(movement.scale_by(enemy.get_speed()));
}
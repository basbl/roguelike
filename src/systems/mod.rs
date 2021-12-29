mod chasing;
mod combat;
mod end_turn;
mod entity_renderer;
mod fov;
mod hud;
mod map_renderer;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(tooltips::tooltips_system())
        .add_system(hud::hud_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(tooltips::tooltips_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(tooltips::tooltips_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

mod entity_renderer;
mod map_renderer;
mod player_input;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_renderer::map_render_system())
        .add_system(entity_renderer::entity_renderer_system())
        .build()
}

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::SpriteRender,
};

use crate::tile_state::TILE_SIZE;

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn init_player(world: &mut World, sprite: SpriteRender) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(TILE_SIZE * 20.0, TILE_SIZE * 11.0, 0.0);

        world
            .create_entity()
            .with(Player)
            .with(transform)
            .with(sprite)
            .build();
    }
}

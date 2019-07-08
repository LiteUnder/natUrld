use amethyst::{
    ecs::{Read, ReadStorage, System, WriteStorage, Join},
    core::transform::Transform,
    core::Float,
    renderer::Camera,
    core::timing::Time,
};

use crate::tile_state::Player;
pub struct PhysicsSystem;

const GRAVITY: f32 = 0.15;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (mut players, mut transforms, time, cam): Self::SystemData) {
        let mut player_transform = Transform::default();

        for(player, transform) in (&mut players, &mut transforms).join() {

            transform.prepend_translation_x(player.velocity[0] * time.time_scale());
            transform.prepend_translation_y(player.velocity[1] * time.time_scale());

            player.falling = transform.translation().y > Float::from_f64(352.0);

            if player.falling {
                player.velocity[1] -= GRAVITY * time.time_scale();
            } else {
                player.velocity[1] = 0.0;
            }

            player_transform = transform.clone()
        }

        for(_, transform) in (&cam, &mut transforms).join() {
            player_transform.set_translation_z(1.0);
            transform.set_translation(*player_transform.translation());
        }
    }
}
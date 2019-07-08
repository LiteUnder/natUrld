use amethyst::{
    ecs::{Read, System, WriteStorage, Join},
    core::transform::Transform,
    input::{InputHandler, StringBindings},
};

use crate::tile_state::Player;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, mut transforms, input): Self::SystemData) {
        for (player, _) in (&mut players, &mut transforms).join() { // currently works because the camera and player are the only two entities with names
            let movement = input.axis_value("horizontal");

            if let Some(mv_x) = movement {
                player.velocity[0] = mv_x as f32 * 3.0;
            }

            let jump = input.action_is_down("jump");

            if jump.unwrap_or(false) {
                println!("{}", player.falling);
                println!("{}", player.velocity[1]);
                if !player.falling {
                    player.velocity[1] = 3.0;
                }
            }
        }
    }
}

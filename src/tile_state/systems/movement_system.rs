use amethyst::{
    ecs::{Read, ReadStorage, System, WriteStorage, Join},
    core::transform::Transform,
    core::Named,
    input::{InputHandler, StringBindings},
    core::timing::Time,
};

//use crate::tile_state::components::Player;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Named>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );

    fn run(&mut self, (names, mut transforms, input, time): Self::SystemData) {
        for (_, transform) in (&names, &mut transforms).join() { // currently works because the camera and player are the only two entities with names
            let movement = input.axis_value("horizontal");

            if let Some(mv_x) = movement {
                let scaled_mv = mv_x as f32 * 3.0 * time.time_scale();
                transform.prepend_translation_x(scaled_mv);
            }
        }
        // TODO: add jumping and basic physics
    }
}

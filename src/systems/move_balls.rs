use crate::components::ball::Ball;
use amethyst::{
    core::{ Time, Transform, },
    derive::SystemDesc,
    ecs::{ System, SystemData, ReadStorage, WriteStorage, Read, Join, },
};

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut local_transforms, time): Self::SystemData) {
        for (ball, local_transform) in (&balls, &mut local_transforms).join() {
            local_transform.prepend_translation_x(ball.velocity.x() * time.delta_seconds());
            local_transform.prepend_translation_y(ball.velocity.y() * time.delta_seconds());
        }
    }
}

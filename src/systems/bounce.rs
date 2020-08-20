use crate::pong::ARENA_HEIGHT;
use crate::{
    components::{ Ball, Paddle, paddle::Side },
    audio::{ self, Sounds, },
};
use amethyst::{
    ecs::{
        System,
        Join,
        WriteStorage,
        Read,
        ReadExpect,
        ReadStorage,
    },
    audio::{ output::Output, Source, },
    assets::AssetStorage,
};
use amethyst::core::Transform;

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (mut balls, paddles, transforms, storage, sounds, audio_output): Self::SystemData) {
        for (ball, ball_transform) in (&mut balls, &transforms).join() {
            let ball_x = ball_transform.translation().x;
            let ball_y = ball_transform.translation().y;

            if (ball_y <= ball.radius && ball.velocity.y() < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity.y() > 0.0)
            {
                ball.velocity.set_y(-ball.velocity.y());
                audio::play_bounce_sound(&*sounds, &storage, audio_output.as_deref());
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - paddle.width * 0.5;
                let paddle_y = paddle_transform.translation().y - paddle.height * 0.5;
                let paddle_collision_dimensions = Rectangle {
                    left: paddle_x - ball.radius,
                    right: paddle_x + paddle.width + ball.radius,
                    bottom: paddle_y - ball.radius,
                    top: paddle_y + paddle.height + ball.radius,
                };
                
                if paddle_collision_dimensions.point_inside(ball_x, ball_y) {
                    if (paddle.side == Side::Left && ball.velocity.x() < 0.0)
                        || (paddle.side == Side::Right && ball.velocity.x() > 0.0)
                    {
                        ball.velocity.set_x(-ball.velocity.x());
                        audio::play_bounce_sound(&*sounds, &storage, audio_output.as_deref());
                    }
                }
            }
        }
    }
}

struct Rectangle {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Rectangle {
    fn point_inside(&self, x: f32, y: f32) -> bool {
        x <= self.right && x >= self.left && y <= self.top && y >= self.bottom
    }
}



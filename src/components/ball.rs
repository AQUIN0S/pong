use amethyst::ecs::{ Component, DenseVecStorage, World, WorldExt, Builder };
use amethyst::renderer::{ SpriteSheet, SpriteRender };
use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use crate::pong::{ ARENA_WIDTH, ARENA_HEIGHT, BALL_VELOCITY_X, BALL_VELOCITY_Y, BALL_RADIUS };

pub const MAX_BALL_VELOCITY_Y: f32 = 100.0;

pub struct Velocity2D {
    x: f32,
    y: f32,
}

impl Velocity2D {
    pub fn new(x: f32, y: f32) -> Velocity2D {
        Velocity2D { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, velocity_x: f32) {
        self.x = velocity_x;
    }

    pub fn set_y(&mut self, velocity_y: f32) {
        self.y = velocity_y.min(MAX_BALL_VELOCITY_Y).max(-MAX_BALL_VELOCITY_Y);
    }

    pub fn _add_x(&mut self, velocity_x: f32) {
        self.x += velocity_x
    }

    pub fn _add_y(&mut self, velocity_y: f32) {
        self.y = (self.y + velocity_y).min(MAX_BALL_VELOCITY_Y).max(-MAX_BALL_VELOCITY_Y);
    }
}

pub struct Ball {
    pub velocity: Velocity2D,
    pub radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            velocity: Velocity2D::new(BALL_VELOCITY_X, BALL_VELOCITY_Y),
            radius: BALL_RADIUS,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_balls(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender::new(spritesheet_handle, 1);

    world.create_entity()
        .with(Ball::default())
        .with(local_transform)
        .with(sprite_render)
        .build();
}

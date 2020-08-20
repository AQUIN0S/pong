use crate::pong::{ PADDLE_WIDTH, PADDLE_HEIGHT, ARENA_WIDTH, ARENA_HEIGHT, };
use amethyst::ecs::{ Component, DenseVecStorage, World, WorldExt, Builder };
use amethyst::renderer::{ SpriteSheet, SpriteRender };
use amethyst::assets::Handle;
use amethyst::core::transform::Transform;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_paddles(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
    let center_y = ARENA_HEIGHT / 2.0;
    let half_paddle_width = PADDLE_WIDTH * 0.5;

    let mut left_transform = Transform::default();
    left_transform.set_translation_xyz(half_paddle_width, center_y, 0.0);

    let mut right_transform = Transform::default();
    right_transform.set_translation_xyz(ARENA_WIDTH - half_paddle_width, center_y, 0.0);

    let sprite_render = SpriteRender::new(spritesheet_handle, 0);

    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(sprite_render.clone())
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(sprite_render)
        .with(right_transform)
        .build(); 
}

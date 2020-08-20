use amethyst::{
    prelude::*,
    core::{
        transform::Transform,
        timing::Time,
    },
    assets::{AssetStorage, Loader, Handle,},
    ecs::{ World, Entity, WorldExt },
    renderer::{
        Camera,
        ImageFormat,
        Texture,
        sprite::{
            SpriteSheet,
            SpriteSheetFormat,
        }
    },
    ui::{ Anchor, TtfFormat, UiText, LineMode, UiTransform, }
};
use crate::{
    components::{ ball, paddle, },
    audio,
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    spritesheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ball_spawn_timer.replace(1.0);
        self.spritesheet_handle.replace(load_sprite_sheet(world));

        paddle::initialize_paddles(world, self.spritesheet_handle.clone().unwrap());
        
        initialize_camera(world);
        initialize_scoreboard(world);
        audio::initialize_audio(world);
    }
 
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                ball::initialize_balls(data.world, self.spritesheet_handle.clone().unwrap());
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: u32,
    pub score_right: u32,
}

pub struct ScoreText {
    pub score_left: Entity,
    pub score_right: Entity,
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(Camera::standard_2d(100.0, 100.0))
        .with(transform)
        .build();
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle, 
        -50., -50., 1., 200., 50.);
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle, Anchor::TopMiddle, 
        50., -50., 1., 200., 50.);

    let score_left = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(), "0".to_string(), [1., 1., 1., 1.], 
            50., LineMode::Single, Anchor::Middle))
        .build();
    
    let score_right = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font, "0".to_string(), [1., 1., 1., 1.,], 
            50., LineMode::Single, Anchor::Middle))
        .build();
    
    world.insert(ScoreText { score_left, score_right });
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", 
        SpriteSheetFormat(texture_handle), 
        (),
        &sprite_sheet_store,
    )
}
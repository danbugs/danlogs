use amethyst::{assets::{AssetStorage, Handle, Loader}, core::transform::Transform, ecs::prelude::{Entity, Component, DenseVecStorage}, prelude::*, renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}, ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode}};
use crate::audio::initialize_audio;

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub struct PikachuVolleyball;
impl SimpleState for PikachuVolleyball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
        let spritesheet = load_spritesheet(world);
        initialize_players(world, spritesheet.clone());
        initialize_pokeball(world, spritesheet.clone());
        initialize_scoreboard(world);
        initialize_audio(world);
    }
}

pub const PLAYER_HEIGHT: f32 = 64.0;
pub const PLAYER_WIDTH: f32 = 64.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

pub struct Player {
    pub side: Side,
    pub width: f32, 
    pub height: f32
}

impl Player {
    fn new(side: Side) -> Player {
        Player {
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_players(world: &mut World, spritesheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = PLAYER_HEIGHT * 0.5;
    left_transform.set_translation_xyz(PLAYER_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PLAYER_WIDTH * 0.5, y, 0.0);
    right_transform.set_rotation_y_axis(std::f32::consts::PI);

    let spriterender = SpriteRender {
        sprite_sheet: spritesheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(spriterender.clone())
        .with(Player::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(spriterender.clone())
        .with(Player::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world
            .read_resource::<AssetStorage<Texture>>();
        
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let spritesheet_store = world
        .read_resource::<AssetStorage<SpriteSheet>>();
    
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &spritesheet_store
    )
}

pub const POKEBALL_VELOCITY: (f32, f32) = (128.0, 0.0);
pub const POKEBALL_RADIUS: f32 = 20.0;

pub struct Pokeball {
    pub velocity: (f32, f32),
    pub radius: f32
}

impl Component for Pokeball {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_pokeball(world: &mut World, spritesheet: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let spriterender = SpriteRender {
        sprite_sheet: spritesheet,
        sprite_number: 1
    };

    world
        .create_entity()
        .with(spriterender)
        .with(Pokeball {
            radius: POKEBALL_RADIUS,
            velocity: POKEBALL_VELOCITY
        })
        .with(local_transform)
        .build();
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(),  // ID
        Anchor::TopMiddle, // anchor
        Anchor::Middle,    // pivot
        -50.,              // x
        -50.,              // y
        1.,                // z
        200.,              // width
        50.,               // height
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::Middle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,            
        ))
        .build();

    world.insert(ScoreText {p1_score, p2_score});
}
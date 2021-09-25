use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::Time;
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};

extern crate rand;

use rand::thread_rng;
use rand::Rng;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_VELOCITY_X: f32 = 10.0;
pub const BALL_VELOCITY_Y: f32 = 5.0;
pub const BALL_RADIUS: f32 = 2.0;
pub const BALL_COUNT: i16 = 500;

#[derive(Default)] //automatically implements the default trait for us
pub struct Pong{
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong{

    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        self.ball_spawn_timer.replace(1.0); //wait 1s before ball is spawned
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        world.register::<Ball>();

        //init_ball(world, self.sprite_sheet_handle.clone().unwrap());
        init_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        init_camera(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        if let Some(mut timer) = self.ball_spawn_timer.take(){
            {
                let time = _data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                for i in 0..BALL_COUNT {
                    init_ball(_data.world, self.sprite_sheet_handle.clone().unwrap());
                }
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent, ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape){
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

/// Setup camera so that screen covers entire arena and (0,0) is bottom left
fn init_camera(World: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    World.create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn init_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>){
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    world.create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    world.create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn init_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>){
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT/2.0, 0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    let mut rng = thread_rng();
    let random_y: f32 = rng.gen_range(-5.0,5.0);
    let random_x: f32 = rng.gen_range(-5.0,5.0);

    world.create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X * random_x, BALL_VELOCITY_Y * random_y],
        })
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet>{
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("sprites/pong_spritesheet.png", ImageFormat::default(), (), &texture_storage)
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load("sprites/pong_spritesheet.ron", SpriteSheetFormat(texture_handle), (), &sheet_store)
    };
    return sheet_handle;
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32
}

impl Paddle{
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT
        }
    }
}

impl Component for Paddle{
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball{
    pub velocity: [f32; 2], //array of two f32 elements
    pub radius: f32,
}

impl Component for Ball{
    type Storage = DenseVecStorage<Self>;
}


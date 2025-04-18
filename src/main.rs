mod animation;
mod platforms;
mod player;

use animation::AnimationPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;

use bevy::{
    prelude::*,
    window::{close_on_esc, WindowResolution},
};
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::Collider,
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);
const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);
const FLOOR_THICKNESS: f32 = 10.0;

const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_WIDTH: f32 = 1024.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND)) // resource added
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0)) // Physics plugin
        .add_plugin(RapierDebugRenderPlugin::default()) // Debug pl
        .add_plugin(PlatformsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
}

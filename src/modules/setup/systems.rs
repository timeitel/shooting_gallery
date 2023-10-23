use super::components::MainCamera;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Background;

// const TOTAL_AMMO: usize = 3;
// const TOTAL_TIME: usize = 90;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(bg_bundle(&server));
    commands.spawn(wood_bundle(&server, (-512., 231.)));
    commands.spawn(wood_bundle(&server, (-250., 231.)));
    commands.spawn(wood_bundle(&server, (100., 231.)));
}

fn wood_bundle(server: &Res<AssetServer>, coords: (f32, f32)) -> (SpriteBundle, Name) {
    let wood = server.load("sprites/wood.png");

    (
        SpriteBundle {
            texture: wood,
            transform: Transform {
                translation: Vec3::new(coords.0, coords.1, 0.),
                ..default()
            },
            ..default()
        },
        Name::new("Wood"),
    )
}

fn bg_bundle(server: &Res<AssetServer>) -> (SpriteBundle, Name) {
    let background = server.load("sprites/background.png");

    (
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280., 720.)),
                ..default()
            },
            texture: background,
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                ..default()
            },
            ..default()
        },
        Name::new("Background"),
    )
}

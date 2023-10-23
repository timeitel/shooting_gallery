use super::components::MainCamera;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Background;

// const TOTAL_AMMO: usize = 3;
// const TOTAL_TIME: usize = 90;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(bg(&server));

    commands.spawn(wood(&server, (-512., 231.)));
    commands.spawn(wood(&server, (-260., 231.)));
    commands.spawn(wood(&server, (-4., 231.)));
    commands.spawn(wood(&server, (251., 231.)));
    commands.spawn(wood(&server, (505., 231.)));

    commands.spawn(straight_curtain(&server, (-512., 319.)));
    commands.spawn(straight_curtain(&server, (-256., 319.)));
    commands.spawn(straight_curtain(&server, (0., 319.)));
    commands.spawn(straight_curtain(&server, (256., 319.)));
    commands.spawn(straight_curtain(&server, (512., 319.)));

    commands.spawn(top_curtain(&server, (-447., 283.)));
    commands.spawn(top_curtain(&server, (-281., 272.)));
    commands.spawn(top_curtain(&server, (-107., 268.)));
    commands.spawn(top_curtain(&server, (-100., 270.)));

    commands.spawn(side_curtain(&server, (-560., 86.), Quat::default()));
    commands.spawn(side_curtain(
        &server,
        (561., 86.),
        Quat::from_rotation_y(3.),
    ));
}

fn wood(server: &Res<AssetServer>, coords: (f32, f32)) -> (SpriteBundle, Name) {
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

fn bg(server: &Res<AssetServer>) -> (SpriteBundle, Name) {
    let image = server.load("sprites/background.png");

    (
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280., 720.)),
                ..default()
            },
            texture: image,
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                ..default()
            },
            ..default()
        },
        Name::new("Background"),
    )
}

fn straight_curtain(server: &Res<AssetServer>, coords: (f32, f32)) -> (SpriteBundle, Name) {
    let image = server.load("sprites/curtain_straight.png");

    (
        SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(coords.0, coords.1, 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Straight"),
    )
}

fn side_curtain(
    server: &Res<AssetServer>,
    coords: (f32, f32),
    rotation: Quat,
) -> (SpriteBundle, Name) {
    let image = server.load("sprites/curtain.png");

    (
        SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(coords.0, coords.1, 5.),
                rotation,
                scale: Vec3 {
                    x: 1.1,
                    y: 1.1,
                    z: 1.,
                },
            },
            ..default()
        },
        Name::new("Curtain Side"),
    )
}

fn top_curtain(
    server: &Res<AssetServer>,
    coords: (f32, f32),
    scale: Option<Vec3>,
) -> (SpriteBundle, Name) {
    let image = server.load("sprites/curtain_top.png");

    (
        SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(coords.0, coords.1, 5.),
                scale: scale.unwrap_or_default(),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    )
}

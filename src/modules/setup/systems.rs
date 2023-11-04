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

    commands.spawn_batch(get_top_curtains(&server));
    commands.spawn_batch(get_side_curtains(&server));
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

fn get_side_curtains(server: &Res<AssetServer>) -> Vec<(SpriteBundle, Name)> {
    let curtain = server.load("sprites/curtain.png");
    let rope = server.load("sprites/curtain_rope.png");

    let left_curtain = (
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(-560., 86., 5.),
                scale: Vec3 {
                    x: 1.1,
                    y: 1.1,
                    z: 1.,
                },
                ..default()
            },
            ..default()
        },
        Name::new("Left curtain"),
    );

    let left_rope = (
        SpriteBundle {
            texture: rope.clone(),
            transform: Transform {
                translation: Vec3::new(-616., 83., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Left curtain rope"),
    );

    let right_curtain = (
        SpriteBundle {
            texture: curtain,
            transform: Transform {
                translation: Vec3::new(561., 86., 5.),
                scale: Vec3 {
                    x: 1.1,
                    y: 1.1,
                    z: 1.,
                },
                rotation: Quat::from_rotation_y(3.),
                ..default()
            },
            ..default()
        },
        Name::new("Right curtain"),
    );

    let right_rope = (
        SpriteBundle {
            texture: rope,
            transform: Transform {
                translation: Vec3::new(612., 83., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Right curtain rope"),
    );

    vec![left_rope, left_curtain, right_rope, right_curtain]
}

fn get_top_curtains(server: &Res<AssetServer>) -> Vec<(SpriteBundle, Name)> {
    let curtain = server.load("sprites/curtain_top.png");
    let mut top_curtains = vec![];
    top_curtains.push((
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(-447., 283., 5.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    ));

    top_curtains.push((
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(-281., 272., 5.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    ));

    top_curtains.push((
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(-107., 268., 5.),
                scale: Vec3::new(1.2, 1.3, 1.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    ));

    top_curtains.push((
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(100., 270., 5.),
                scale: Vec3::new(1.2, 1.3, 1.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    ));

    top_curtains.push((
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(300., 270., 5.),
                scale: Vec3::new(1.2, 1.2, 1.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    ));

    top_curtains.push((
        SpriteBundle {
            texture: curtain.clone(),
            transform: Transform {
                translation: Vec3::new(500., 270., 5.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    ));

    top_curtains
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
                scale: scale.unwrap_or(Vec3::new(1., 1., 1.)),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Top"),
    )
}

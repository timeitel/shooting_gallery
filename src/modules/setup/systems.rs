use super::components::MainCamera;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Background;

// const TOTAL_AMMO: usize = 3;
// const TOTAL_TIME: usize = 90;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(get_bg(&server));
    commands.spawn_batch(get_wood_boards(&server));
    commands.spawn_batch(get_straight_curtains(&server));
    commands.spawn_batch(get_top_curtains(&server));
    commands.spawn_batch(get_side_curtains(&server));
}

fn get_wood_boards(server: &Res<AssetServer>) -> Vec<(SpriteBundle, Name)> {
    let wood = server.load("sprites/wood.png");
    let mut boards = vec![];

    let new_bundle = |x: f32, y: f32, z: f32| -> (SpriteBundle, Name) {
        (
            SpriteBundle {
                texture: wood.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    ..default()
                },
                ..default()
            },
            Name::new("Wood"),
        )
    };

    boards.push(new_bundle(-512., 231., 0.));
    boards.push(new_bundle(-260., 231., 0.));
    boards.push(new_bundle(-4., 231., 0.));
    boards.push(new_bundle(251., 231., 0.));
    boards.push(new_bundle(505., 231., 0.));

    boards
}

fn get_bg(server: &Res<AssetServer>) -> (SpriteBundle, Name) {
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

fn get_straight_curtains(server: &Res<AssetServer>) -> Vec<(SpriteBundle, Name)> {
    let image = server.load("sprites/curtain_straight.png");
    let mut bundles = vec![];

    bundles.push((
        SpriteBundle {
            texture: image.clone(),
            transform: Transform {
                translation: Vec3::new(-512., 319., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Straight"),
    ));
    bundles.push((
        SpriteBundle {
            texture: image.clone(),
            transform: Transform {
                translation: Vec3::new(-256., 319., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Straight"),
    ));

    bundles.push((
        SpriteBundle {
            texture: image.clone(),
            transform: Transform {
                translation: Vec3::new(0., 319., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Straight"),
    ));

    bundles.push((
        SpriteBundle {
            texture: image.clone(),
            transform: Transform {
                translation: Vec3::new(256., 319., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Straight"),
    ));

    bundles.push((
        SpriteBundle {
            texture: image,
            transform: Transform {
                translation: Vec3::new(512., 319., 10.),
                ..default()
            },
            ..default()
        },
        Name::new("Curtain Straight"),
    ));

    bundles
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

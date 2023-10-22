use super::components::MainCamera;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Background;

// const TOTAL_AMMO: usize = 3;
// const TOTAL_TIME: usize = 90;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let texture = server.load("sprites/background.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(640., 480.)),
                ..default()
            },
            texture,
            ..default()
        },
        Name::new("Background"),
    ));
}

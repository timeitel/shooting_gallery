use super::components::MainCamera;
use bevy::prelude::*;

const TOTAL_AMMO: usize = 3;
const TOTAL_TIME: usize = 90;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn setup_stall(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/bg_wood.png"),
        ..default()
    });
}

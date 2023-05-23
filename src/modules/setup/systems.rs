use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

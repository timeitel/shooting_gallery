use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        .add_startup_system(setup)
        .add_system(spawn_ball)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background_0003.png"),
        ..default()
    });
}

fn spawn_ball(mut commands: Commands, input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>) {
    if input.just_pressed(KeyCode::Space) {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("sprites/background_0003.png"),
            ..default()
        });
    }
}

use bevy::prelude::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "It's a shooting gallery!!".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_stall);
    }
}

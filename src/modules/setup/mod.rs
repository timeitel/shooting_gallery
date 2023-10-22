use bevy::prelude::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "It's a shooting gallery!!".into(),
                resolution: (640., 480.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup);
    }
}

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
                resolution: (1250., 720.).into(),
                resizable: true,
                position: WindowPosition::At(IVec2::new(100, 1000)), // TODO: remove
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup);
    }
}

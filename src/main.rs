mod modules;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use modules::setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins((
            SetupPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ))
        .run();
}

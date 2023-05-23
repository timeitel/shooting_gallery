mod modules;

use bevy::prelude::*;
use modules::setup::SetupPlugin;

fn main() {
    App::new().add_plugin(SetupPlugin).run();
}

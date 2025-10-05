mod components;
mod systems;
mod plugins;

use bevy::prelude::*;
use bevy::app::App;

use crate::plugins::{CameraPlugin, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
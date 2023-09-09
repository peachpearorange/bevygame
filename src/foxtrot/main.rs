// disable console on windows for release builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use {bevy::prelude::*, GamePlugin};

fn main() { App::new().add_plugin(GamePlugin).run(); }

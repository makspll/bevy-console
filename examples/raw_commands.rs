use bevy::prelude::*;
use bevy_console::{ConsoleCommandEntered, ConsolePlugin, ConsoleSet};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .add_systems(Startup, setup_camera_system)
        .add_systems(Update, raw_commands.in_set(ConsoleSet::Commands))
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn raw_commands(mut console_commands: MessageReader<ConsoleCommandEntered>) {
    for ConsoleCommandEntered { command_name, args } in console_commands.read() {
        println!(r#"Entered command "{command_name}" with args {:#?}"#, args);
    }
}

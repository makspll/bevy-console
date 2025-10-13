use bevy::prelude::*;
use bevy_console::{ConsolePlugin, ConsoleSet, PrintConsoleLine};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .add_systems(Startup, setup_camera_system)
        // NOTE: this wouldn't work for this particular case,
        // systems in the [`ConsoleSet::Commands`] do not run if there are no console commands entered
        // .add_systems(Update, write_to_console.in_set(ConsoleSet::Commands))
        // the below is the equivalent but without run conditions
        .add_systems(Update, write_to_console.after(ConsoleSet::ConsoleUI))
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn write_to_console(mut console_line: MessageWriter<PrintConsoleLine>) {
    console_line.write(PrintConsoleLine::new("Hello".into()));
}

use bevy::input::keyboard::NativeKeyCode;
use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .add_systems(Startup, setup_camera_system)
        .insert_resource(ConsoleConfiguration {
            keys: vec![
                // Console key on a swedish keyboard
                KeyCode::Unidentified(NativeKeyCode::Android(41)),
                KeyCode::Unidentified(NativeKeyCode::MacOS(41)),
                KeyCode::Unidentified(NativeKeyCode::Windows(41)),
                KeyCode::Unidentified(NativeKeyCode::Xkb(41)),
                // US console key
                KeyCode::Backquote,
                // F1 key
                KeyCode::F1,
            ],
            ..Default::default()
        })
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

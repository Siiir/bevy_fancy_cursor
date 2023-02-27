use bevy::app::App;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Plugin does its job in `bevy::app::StartupStage::PostStartup` when camera is already spawned.
        .add_plugin(
            bevy_fancy_cursor::CursorSettings::basic()
                .img_path("fancy cursor.png")
                .size(Size::new(Val::Px(50.0), Val::Px(70.0)))
                .build()
        )
        // You can spawn camera in `bevy::app::StartupStage::Startup`, which is before (AND MUST BE) `PostStartup`.
        .add_startup_system(spawn_camera)
        .run()
}

fn spawn_camera(mut commands: Commands){
    commands.spawn((
        bevy_fancy_cursor::UserCamera, // Obligatory marker for user camera. Other cameras will be ignored and won't get special cursor.
        Camera2dBundle::default(),
    ));
}
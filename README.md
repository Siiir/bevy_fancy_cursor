# bevy_fancy_cursor
Plagin for bevy. Uses bevy's `ImageBundle` to crate a custom cursor from file with image.

## Contribute
 if you want to see project growing and `FancyCursor` becoming more generic.

## Instalation
#### bevy_fancy_cursor = { git = "https://github.com/Siiir/bevy_fancy_cursor" }
Until I find some time to chat with bevy team and made better docs this mini-project won't be published on crates.io.  
You can help me in this though.

## Example
```
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
```

# Contact
tomasz_nehring@outlook.com

## PS
Sorry, if I harmed English language anywhere.



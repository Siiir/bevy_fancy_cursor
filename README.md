# bevy_fancy_cursor
Facilitates **creating a custom cursor** in bevy-based app. Uses bevy's `ImageBundle` to replace the original cursor with a moving UI element.


## Instalation
From inside of your project
* run:  
`cargo add siiir-bevy_fancy_cursor`
* **or** add to manifest file (Cargo.toml) under `[dependencies]`  
```bevy_fancy_cursor = { git = "https://github.com/Siiir/bevy_fancy_cursor" }```


## Example usage
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
        // You can spawn camera in `bevy::app::StartupStage::Startup`, which is (AND MUST BE) BEFORE `PostStartup`.
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

### Produces this app with a fancy cursor from the "./assets" folder.  
![Image picturing resulting app.](https://raw.githubusercontent.com/Siiir/bevy_fancy_cursor/master/Example%20with%20fancy%20cursor.png)

### To run the above example with contained asset.
Download the [version](https://github.com/Siiir/bevy_fancy_cursor/releases/tag/v0.1.0-beta) I used when creating it.
`cargo run` it. Enjoy.


## [Documentation](https://docs.rs/siiir-bevy_fancy_cursor/latest/bevy_fancy_cursor/)


## Contribute
if you want to see project growing and `FancyCursor` becoming more generic.


## Contact
tomasz_nehring@outlook.com


//! Crate that uses `ImageBundle` to replace default cursor with a fancy one instantiated from image file.

use bevy::{prelude::*, render::camera::RenderTarget, ui::FocusPolicy};
use derive_more::{From, AsRef, AsMut};

/// Default marker for entity that servers as user camera with fancy cursor.
#[derive(Copy, Clone, Default)] // Constructors
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UserCamera;

/// Default marker for entity with `ImageBundle` that servers as a custom cursor.
#[derive(Copy, Clone, Default)] // Constructors
#[derive(Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UserCursor;

#[derive(Clone, Default)]
#[derive(Debug)]
pub struct CursorSettings<P= &'static str, Curs = UserCursor, Cam= UserCamera>
    where
        P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static,
        Curs: Component + Send + Sync + Clone + 'static,
        Cam: Component + Send + Sync + Clone + 'static, {
    pub img_path: P,
    pub size: Size,
    pub render_offset: Vec2,
    pub z_index: ZIndex,
    pub cursor_marker: Curs,
    pub _phantom_camera: std::marker::PhantomData<Cam>,
}

impl<P, Curs, Cam> CursorSettings<P, Curs, Cam>  where
        P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static,
        Curs: Component + Send + Sync + Clone + 'static,
        Cam: Component + Send + Sync + Clone + 'static, {
    pub fn build(self)-> FancyCursor<P, Curs, Cam>{
        FancyCursor::from(self)
    }
    pub fn img_path(mut self, path: P) -> Self {
        self.img_path= path;
        self
    }
    pub fn size(mut self, size: Size) -> Self{
        self.size= size;
        self
    }
    pub fn render_offset(mut self, offset: Vec2) -> Self {
        self.render_offset= offset;
        self
    }
    pub fn z_index(mut self, z_index: ZIndex)-> Self{
        self.z_index= z_index;
        self
    }
    pub fn cursor_marker(mut self, marker: Curs)-> Self{
        self.cursor_marker= marker;
        self
    }
}
impl<P> CursorSettings<P> where
        P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static, {
    pub fn from_image(path: P) -> Self {
        Self{
            img_path: path,
            size: Size::new(Val::Px(36.0), Val::Px(36.0)),
            render_offset: Vec2::ZERO,
            z_index: ZIndex::Global(15),
            cursor_marker: UserCursor,
            _phantom_camera: default(),
        }
    }
}

impl CursorSettings<&str, UserCursor, UserCamera> {
    /// Alternative to `Default::default` which doesn't infer types well.
    /// This will coerce generic type parameters to default types.
    pub fn basic() -> Self {
        Self::from_image(r"texture\fancy cursor.png")
    }
}

/// Bevy plugin that “replaces” default cursor with one created from image.
///
/// Entity with `Camera` & `Cam` components must be spawned during startup before `StartupStage::PostStartup`,
/// So remember to `app.add_startup_system([system that spawns camera with `Cam` component])` .
///
/// Internally plugin uses `Query<&Camera, With<Cam>>` each frame and on startup to get camera rendering target.
/// If that target isn't `bevy::render::camera::RenderTarget::Window`, primary window will be used instead.
#[derive(Clone, From, Default)]
#[derive(Deref, DerefMut, AsRef, AsMut)]
#[derive(Debug)]
pub struct FancyCursor<P= &'static str, Curs = UserCursor, Cam= UserCamera>
    where
        P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static,
        Curs: Component + Send + Sync + Clone + 'static,
        Cam: Component + Send + Sync + Clone + 'static, {
    pub settings: std::sync::Arc<CursorSettings<P, Curs, Cam>>,
}

impl<P, Curs, Cam> Plugin for FancyCursor<P, Curs, Cam>
    where
        P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static,
        Curs: Component + Send + Sync + Clone + 'static,
        Cam: Component + Send + Sync + Clone + 'static,
{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, self.create_setup())
            .add_system(self.create_update())
        ;
    }
}

impl<P, Curs, Cam> FancyCursor<P, Curs, Cam>
    where
        P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static,
        Curs: Component + Send + Sync + Clone + 'static,
        Cam: Component + Send + Sync + Clone + 'static,
{
    pub fn create_setup(&self) -> impl Fn(
        ResMut<Windows>, Res<AssetServer>,
        Commands,
        Query<&Camera, With<Cam>>,
    ) {
        let curs_settings= std::sync::Arc::clone(&self.settings);

        move |mut wins, asset_serv, mut commands, camera| {
            let camera= camera.single();
            // get the window that the camera is displaying to (or the primary window)
            let win = if let RenderTarget::Window(win_id) = camera.target {
                wins.get_mut(win_id).unwrap()
            } else {
                wins.get_primary_mut().unwrap()
            };
            win.set_cursor_visibility(false);
            // Instead of default cursor we have.
            commands.spawn((
                ImageBundle {
                    style: Style {
                        position: UiRect::all(Val::Auto),
                        position_type: PositionType::Absolute,
                        size: curs_settings.size.clone(),
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    image: UiImage(asset_serv.load(curs_settings.img_path.clone())),
                    z_index: curs_settings.z_index.clone(),
                    ..default()
                },
                curs_settings.cursor_marker.clone(),
            ));
        }
    }

    pub fn create_update(&self) -> impl Fn(
        Res<Windows>,
        Query<&Camera, With<Cam>>,
        Query<(&mut Style, &mut Visibility), With<Curs>>,
    ) {
        let settings= std::sync::Arc::clone(&self.settings);

        move |wins: Res<Windows>, camera: Query<&Camera, With<Cam>>, mut cursor: Query<(&mut Style, &mut Visibility), With<Curs>>| {
            let camera = camera.single();

            // get the window that the camera is displaying to (or the primary window)
            let win = if let RenderTarget::Window(win_id) = camera.target {
                wins.get(win_id).unwrap()
            } else {
                wins.get_primary().unwrap()
            };
            let (mut img_style, mut visibility) = cursor.single_mut();

            // check if the cursor is inside the window and get its position
            if let Some(screen_pos) = win.cursor_position() {
                img_style.position.left = Val::Px(screen_pos.x +settings.render_offset.x);
                img_style.position.top = Val::Px(win.height() - (screen_pos.y + settings.render_offset.y));
                *visibility = Visibility::VISIBLE;
            } else {
                img_style.position.left = Val::Px(-100.);
                img_style.position.top = Val::Px(-100.);
                *visibility = Visibility::INVISIBLE;
            }
        }
    }
}

impl<P, Curs, Cam> From<CursorSettings<P, Curs, Cam>> for FancyCursor<P, Curs, Cam> where
    P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static,
    Curs: Component + Send + Sync + Clone + 'static,
    Cam: Component + Send + Sync + Clone + 'static,{
    fn from(settings: CursorSettings<P, Curs, Cam>) -> Self {
        Self{ settings: std::sync::Arc::new(settings) }
    }
}

impl<P> FancyCursor<P> where P: Into<bevy::asset::AssetPath<'static>> + Send + Sync + Clone + 'static{
    pub fn from_image(path: P) -> Self {
        Self::from(CursorSettings::from_image(path))
    }
}
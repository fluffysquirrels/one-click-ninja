use bevy::prelude::*;

const WIN_W: f32 = 800.;
const WIN_H: f32 = 600.;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "One-Click Ninja".to_string(),
            width: WIN_W,
            height: WIN_H,
            .. Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tex = asset_server.load("sword.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(tex.into()),
        transform: Transform {
            translation: Vec3::new(-200., 100., 0.),
            scale: Vec3::new(0.3, 0.3, 0.3),
            .. Default::default()
        },
        .. Default::default()
    });
}

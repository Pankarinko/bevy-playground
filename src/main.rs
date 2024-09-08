use bevy::{
    color::palettes::css::{GREEN, YELLOW},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(YELLOW.into()))
        .add_systems(Startup, spawn_rect)
        .run();
}

fn spawn_rect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        material: materials.add(Color::from(GREEN)),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        ..default()
    });
}

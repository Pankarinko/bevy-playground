use bevy::{
    color::palettes::css::{GREEN, YELLOW},
    math::VectorSpace,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::WindowBackendScaleFactorChanged,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(YELLOW.into()))
        .add_systems(Startup, spawn_rect)
        .add_systems(Update, move_rect)
        .run();
}

#[derive(Component)]
struct Shape {
    spawn: Vec3,
}

impl Shape {
    fn new(spawn: Vec3) -> Self {
        Shape { spawn }
    }
}

fn spawn_rect(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let spawn = Vec3::ZERO;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(Color::from(GREEN)),
            transform: Transform::from_translation(spawn).with_scale(Vec3::splat(128.)),
            ..default()
        },
        Shape::new(spawn),
    ));
    commands.spawn(Camera2dBundle::default());
}

fn move_rect(mut shapes: Query<(&mut Transform, &mut Shape)>, timer: Res<Time>) {
    for (mut transform, mut shape) in &mut shapes {
        let direction = transform.local_x();
        transform.translation += direction * 36.0 * timer.delta_seconds();
    }
}

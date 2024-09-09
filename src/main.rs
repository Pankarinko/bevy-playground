use std::num;

use bevy::{
    color::palettes::css::{GREEN, YELLOW},
    math::VectorSpace,
    prelude::*,
    scene::ron::value::Float,
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
    speed: i16,
}

impl Shape {
    fn new(spawn: Vec3) -> Self {
        Shape { spawn, speed: 180 }
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
        if (transform.translation.x - shape.spawn.x).abs() < 180.0 {
            move_horizontal(&mut transform, &shape, &timer);
        } else {
            move_vertical(&mut transform, &shape, &timer);
            if (transform.translation.y - shape.spawn.y).abs() >= 180.0 {
                shape.speed = -shape.speed;
                move_horizontal(&mut transform, &shape, &timer);
            }
        }
    }
}

fn move_horizontal(transform: &mut Mut<'_, Transform>, shape: &Mut<'_, Shape>, timer: &Res<Time>) {
    let direction = transform.local_x();
    transform.translation += direction * shape.speed.into() * timer.delta_seconds();
}

fn move_vertical(transform: &mut Mut<'_, Transform>, shape: &Mut<'_, Shape>, timer: &Res<Time>) {
    let direction = transform.local_y();
    transform.translation += direction * shape.speed.into() * timer.delta_seconds();
}

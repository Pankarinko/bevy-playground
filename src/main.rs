use std::num;

use bevy::{
    animation::{AnimationTarget, AnimationTargetId},
    color::palettes::css::{GREEN, YELLOW},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(YELLOW.into()))
        .run();
}

#[derive(Component)]
struct Shape {
    spawn: Vec3,
}
#[derive(Bundle)]
struct MovingShapeBundle {
    shape: Shape,
}

fn transform() {
    let mut animation = AnimationClip::default();
}

pub use bevy::prelude::*;
use strum_macros::Display;


#[derive(Component, Reflect, Display)]
#[reflect(Component)]
pub enum Wheel {
    Left,
    Right,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GltfNodeRoot;

// #[derive(Component, Reflect, Default)]
// pub struct GizmoFocused;

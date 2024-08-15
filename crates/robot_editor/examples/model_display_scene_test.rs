//! test for the positioning of displaying models
//!

use bevy::{prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use robot_editor::{
    model_display::{
        plugins::{ModelDisplayerPlugin},
        systems::display_model,
    },
    shaders::{neon_glow::NeonGlowMaterial, plugins::CustomShadersPlugin},
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(CustomShadersPlugin)
        .add_plugins(ModelDisplayerPlugin)
        .add_systems(Startup, display_model_test)
        .run();
}

pub fn display_model_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut neon_materials: ResMut<Assets<NeonGlowMaterial>>,
) {
    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh());
    display_model(&mut commands, &mut neon_materials, mesh_handle);
}

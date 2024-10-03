use app_core::{plugins::AppSourcesPlugin};
use bevy::{
    prelude::*,
};
use bevy_camera_extras::ObservedBy;
use bevy_mod_raycast::cursor::CursorRayPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_extras::prelude::{
    mesh::{GeometryFile, GeometryFlag}, DeserializeAssetFrom, SerializationBasePlugin, SerializationPhysicsPlugin, SerializationPlugin
};
use bevy_serialization_urdf::{
    plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin},
};
use bevy_ui_extras::*;
use robot_editor::{model_display::components::DisplayModel, plugins::*, resources::RobotControls, states::RobotEditorState};

pub fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(CursorRayPlugin)
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .insert_state(RobotEditorState::Active)
        // robot editor
        .add_plugins(RobotEditorPlugin)
        .add_plugins(UiExtrasDebug {
            ui_style: UiStyle::Default,
            ..default()
        })

        // serialization plugins
        .add_plugins(SerializationPlugin)
         //.add_plugins(SerializationBasePlugin)
        .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFile, Mesh>::default())

        .add_plugins(SerializationPhysicsPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // world setup
        // .add_systems(
        //     Update,
        //     visualize_entities_with_component::<DisplayModel>(bevy_ui_extras::Display::Side(
        //         bevy_ui_extras::Side::Right,
        //     )),
        // )
        // .add_systems(Update, visualize_entities_with_component::<ObservedBy>(bevy_ui_extras::Display::Side(bevy_ui_extras::Side::Right)))
        .add_systems(Update, visualize_resource::<RobotControls>(bevy_ui_extras::Display::Window))

        .add_systems(Startup, setup_editor_area)
        //.add_systems(Update, display_model_image_to_file)
        .run();
}

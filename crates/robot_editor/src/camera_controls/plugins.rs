use bevy::prelude::*;
use bevy_camera_extras::*;
use bevy_ui_extras::visualize_entities_with_component;

use crate::raycast_utils::resources::CursorRayHits;

/// camera controls for robot editor camera
pub struct RobotEditorCameraPlugin;

impl Plugin for RobotEditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(CameraExtrasPlugin::default())
        //.add_systems(Update, set_cam_to_watch)
        .add_systems(Update, visualize_entities_with_component::<ObservedBy>(bevy_ui_extras::Display::Side(bevy_ui_extras::Side::Right)))
        .add_systems(Update, click_camera_focus_target)
        //.add_systems(PreUpdate, click_camera_focus_target)
        ;
    }
}

// pub fn set_cam_to_watch(
//     flycams: Query<(Entity, &CameraMode), Without<Viewer>>,
//     mut commands: Commands,
// ) {
//     for (e, _) in flycams.iter() {
//         commands.entity(e).insert(Viewer::default())
//         ;
//     }
// }

///click a target to focus camera on
pub fn click_camera_focus_target(
    cursor_ray_hits: Res<CursorRayHits>,
    mesh_query: Query<(Entity, &Handle<Mesh>)>,
    observed_bodies: Query<&ObservedBy>,
    mouse: ResMut<ButtonInput<MouseButton>>,
    keys: ResMut<ButtonInput<KeyCode>>,
    cameras: Query<Entity, With<CameraMode>>,
    mut commands: Commands,
) {
    if mouse.just_pressed(MouseButton::Right) && keys.pressed(KeyCode::ShiftLeft){
        let Some((_, _, (e, _))) = cursor_ray_hits.first_with(&mesh_query) else {return;};
        // get_first_hit_with(
        //     &**cursor_ray_hits
        //     , &mesh_query
        // ) else {return;};
        if observed_bodies.contains(e) {
            commands.entity(e).remove::<ObservedBy>();
        } else {
            let Ok(camera) = cameras.get_single() else {
                warn!("multiple cameras detected. Can't set focus target. Aborting");
                return;
            };
            commands.entity(e).insert(ObservedBy(camera));
        }
    }
}
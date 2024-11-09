use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_mod_outline::{OutlineBundle, OutlineVolume};
use bevy_mod_picking::{
    focus::PickingInteraction, highlight::PickHighlight, picking_core::{Pickable, PickingPluginsSettings},
    selection::PickSelection, PickableBundle,
};
use bevy_serialization_extras::prelude::{link::StructureFlag, rigidbodies::RigidBodyFlag};
use transform_gizmo_bevy::GizmoTarget;

pub fn toggle_picking_enabled(
    gizmo_targets: Query<&GizmoTarget>,
    mut picking_settings: ResMut<PickingPluginsSettings>,
) {
    // Picking is disabled when any of the gizmos is focused or active.

    picking_settings.is_enabled = gizmo_targets
        .iter()
        .all(|target| !target.is_focused() && !target.is_active());
}

pub fn update_picking(
    mut commands: Commands,
    mut keys: Res<ButtonInput<KeyCode>>,
    mut targets: Query<(
        Entity,
        Option<&StructureFlag>,
        &PickSelection,
    ), Changed<PickSelection>>,
    mut rigid_bodies: Query<&mut RigidBodyFlag>,
    mut outlines: Query<&mut OutlineVolume>,
) {
    // Continuously update entities based on their picking state

    for (e, _, selectable) in &mut targets {
        if selectable.is_selected {
            let mut entity_cmd = commands.entity(e);

            entity_cmd.insert(GizmoTarget::default());
            
            let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Fixed);
            let _ = outlines.get_mut(e).map(|mut outline| outline.visible = true);

            if keys.pressed(KeyCode::ShiftLeft) {
                
            }
        } else {
            let mut entity_cmd = commands.entity(e);

            entity_cmd.remove::<GizmoTarget>();
            
            let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Dynamic);
            let _ = outlines.get_mut(e).map(|mut outline| outline.visible = false);
        }
    }

    //TODO: re-attempt to implement this to auto-freeze attached components
    // for (entity, structure) in &mut targets {
    //     let Ok(selectable) = selectables.get(entity) else {return;};
    //     //let Some((hit, ..)) = hits.0.first() else {return;};
    //     if selectable.is_selected {
    //         let mut total_targets = Vec::new();
        
    //         // add all connected structures to list
    //         if let Some(structure) = structure {
    //             structures.iter()
    //             .filter(|(_, candidate)| candidate.name == structure.name)
    //             .for_each(|target| total_targets.push(target.0));
    //         } 
    //         total_targets.push(entity);
            
    //         println!("total targets: {:#?}", total_targets);
    //         for e in total_targets {
    //             let mut entity_cmd = commands.entity(e);

    //             entity_cmd.insert(GizmoTarget::default());
                
    //             let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Fixed);
    //             let _ = outlines.get_mut(e).map(|mut outline| outline.visible = true);
    //             if let Ok(mut selectable) = selectables.get_mut(e) {
    //                 selectable.is_selected = true;
    //             }
    //         }
            

    //     } else if selectable.is_selected == false {
    //         println!("unpicking");
    //         let mut total_targets = Vec::new();
        
    //         // add all connected structures to list
    //         if let Some(structure) = structure {
    //             structures.iter()
    //             .filter(|(_, candidate)| candidate.name == structure.name)
    //             .for_each(|target| total_targets.push(target.0));
    //         } 
    //         total_targets.push(entity);
            
    //         for e in total_targets {
    //             let mut entity_cmd = commands.entity(e);

    //             entity_cmd.remove::<GizmoTarget>();
                
    //             let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Fixed);
    //             let _ = outlines.get_mut(e).map(|mut outline| outline.visible = false);
    //         }
            
    //     }
    // }
}


pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (With<StructureFlag>, Without<Pickable>)>,
) {
    for e in models_query.iter() {
        commands.entity(e).insert((
            PickableBundle {
                pickable: Pickable::default(),
                interaction: PickingInteraction::default(),
                selection: PickSelection::default(),
                highlight: PickHighlight::default(),
            },
            OutlineBundle {
                outline: OutlineVolume {
                    visible: false,
                    colour: Color::Srgba(Srgba::GREEN),
                    width: 2.0,
                },
                ..default()
            },
        ));
    }
}

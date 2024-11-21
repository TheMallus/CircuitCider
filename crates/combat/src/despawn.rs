use bevy::prelude::*;
use crate::components::{Health, Bullet};
pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > 10.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
// fn despawn_entities(mut commands: Commands, query: Query<(Entity, &Health), Without<Bullet>>) {
//     for (entity, health) in query.iter() {
//         if health.hp <= 0.0 {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }
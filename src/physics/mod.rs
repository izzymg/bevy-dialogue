use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut trans, velocity) in query.iter_mut() {
        trans.translation = trans.translation + (velocity.0 * time.delta_seconds()); 
    }
}
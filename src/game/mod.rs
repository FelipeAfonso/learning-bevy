use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::entities::{EnemyEntity, PlayerAttached, PlayerEntity};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(detect_intersection);
    }
}

pub fn detect_intersection(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &Sprite, Entity), With<EnemyEntity>>,
    player_query: Query<
        (&Transform, &Sprite, Entity),
        (With<PlayerEntity>, Without<PlayerAttached>),
    >,
) {
    for player in player_query.iter() {
        let player_pos = player.0.translation;
        let player_size = player.1.custom_size.unwrap_or_default();
        let player_entity = player.2;
        for enemy in enemy_query.iter() {
            let enemy_pos = enemy.0.translation;
            let enemy_size = enemy.1.custom_size.unwrap_or_default();
            let enemy_entity = enemy.2;
            match collide(player_pos, player_size, enemy_pos, enemy_size) {
                Some(Collision::Bottom) => {
                    commands.entity(player_entity).despawn();
                }
                Some(_collision) => {
                    commands.entity(enemy_entity).despawn();
                }
                _ => {}
            };
        }
    }
}

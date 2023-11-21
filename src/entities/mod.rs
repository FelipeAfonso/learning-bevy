use std::task::Wake;

use bevy::prelude::*;

use crate::controllers::PlayerControllerState;

#[derive(Component)]
pub struct PlayerEntity;

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(move_player);
    }
}

pub fn rotate_player(time: Res<Time>, mut query: Query<&mut Transform, With<PlayerEntity>>) {
    for mut player in &mut query {
        player.rotate_y(3.0 * time.delta_seconds())
    }
}

pub fn move_player(
    time: Res<Time>,
    state: Res<PlayerControllerState>,
    mut query: Query<&mut Transform, With<PlayerEntity>>,
) {
    let st = state.get_state();
    //println!(" -- x: {} -- y: {} --", st.0, st.1);
    let speed: f32 = if state.is_boosting() { 4.0 } else { 2.0 };

    for mut player in &mut query {
        player.translation.x += st.0 * speed * time.delta_seconds();
        player.translation.z += st.1 * speed * time.delta_seconds();
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn((
        PlayerEntity,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.5,
                height: 2.0,
                resolution: 16,
                segments: 2,
            })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

use bevy::prelude::*;
mod greeter;

#[derive(Resource)]
struct RotateTimer(Timer);

#[derive(Component)]
pub struct CubeEntity;

fn main() {
    App::new()
        .insert_resource(RotateTimer(Timer::from_seconds(0.05, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        //.add_plugin(greeter::HelloPlugin)
        .add_startup_system(setup)
        .add_system(rotate_cube)
        .run();
}

fn rotate_cube(
    time: Res<Time>,
    mut timer: ResMut<RotateTimer>,
    mut query: Query<&mut Transform, With<CubeEntity>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut cube in &mut query {
            cube.rotate_y(10.0 * time.delta_seconds())
        }
    }
}

fn setup(
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
        CubeEntity,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

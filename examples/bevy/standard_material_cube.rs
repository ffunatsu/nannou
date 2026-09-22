use bevy::prelude::*;
// use nannou::NannouPlugin;

fn main() {
    App::new()
        // .add_plugins((DefaultPlugins, NannouPlugin))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.5, 2.5, 2.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.1),
            ..default()
        })),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            intensity: 1_500_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 4.0),
    ));
}

fn rotate_cube(time: Res<Time>, mut cubes: Query<&mut Transform, With<Mesh3d>>) {
    for mut transform in &mut cubes {
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs() * 0.7)
            * Quat::from_rotation_x(time.elapsed_secs() * 0.4);
    }
}

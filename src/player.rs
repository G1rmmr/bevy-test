use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_movement_system)
        .run();
}

#[derive(Component)]
struct FPSCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FPSCamera,
    ));

    commands.spawn_bundle(PbrBundle {
        mesh: bevy::prelude::Mesh::from(shape::Cube { size: 2.0 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut direction = Vec3::ZERO;
    let speed = 5.0;

    if keyboard_input.pressed(KeyCode::W) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    for mut transform in query.iter_mut() {
        transform.translation += direction * speed * time.delta_seconds();
    }
}

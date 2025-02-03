use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_movement_system) // 카메라 이동 시스템 등록
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

    // 예시로 3D 큐브를 하나 추가
    commands.spawn_bundle(PbrBundle {
        mesh: bevy::prelude::Mesh::from(shape::Cube { size: 2.0 }),
        material: StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    // 간단한 조명 추가
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

// 카메라 이동을 위한 시스템
fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut direction = Vec3::ZERO;
    let speed = 5.0;

    // W/S: 전후방 이동 (z축)
    if keyboard_input.pressed(KeyCode::W) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.z += 1.0;
    }
    // A/D: 좌우 이동 (x축)
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    // 방향 벡터가 0이 아니라면 정규화하여 일정 속도로 이동
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    // 모든 카메라에 대해 이동 적용
    for mut transform in query.iter_mut() {
        transform.translation += direction * speed * time.delta_seconds();
    }
}

use bevy::prelude::*;

struct Rotator;

impl Rotator{
    fn rotator(
        mut query: Query<&mut Transform, With<Rotator>>,
        keys: Res<Input<KeyCode>>,
    ){
        for mut item in query.iter_mut() {
            // x rotation
            if keys.pressed(KeyCode::Q) {
                item.rotate(Quat::from_rotation_x(0.02));
            } else if keys.pressed(KeyCode::E) {
                item.rotate(Quat::from_rotation_x(-0.02));
            }
            // y rotation
            if keys.pressed(KeyCode::D) {
                item.rotate(Quat::from_rotation_y(0.02));
            } else if keys.pressed(KeyCode::A) {
                item.rotate(Quat::from_rotation_y(-0.02));
            }
            // z rotation
            if keys.pressed(KeyCode::W) {
                item.rotate(Quat::from_rotation_z(0.02));
            } else if keys.pressed(KeyCode::S) {
                item.rotate(Quat::from_rotation_z(-0.02));
            }
        }
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // change window position
    let window = windows.get_primary_mut().unwrap();
    window.set_position([0, 0].into());

    // create the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);

    // cube
    commands.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    })
    .insert(Rotator);

    // light source
    commands.spawn_bundle(LightBundle{
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 }) // don't actually know what this does but I should find out
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(Rotator::rotator.system())
        .run();
}

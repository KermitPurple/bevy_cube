use bevy::prelude::*;

struct Rotator;

impl Rotator{
    fn rotator(mut query: Query<&mut Transform, With<Rotator>>){
        for mut item in query.iter_mut() {
            item.rotate(Quat::from_rotation_x(0.1));
            item.rotate(Quat::from_rotation_y(0.05));
            item.rotate(Quat::from_rotation_z(0.025));
        }
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(Rotator::rotator.system())
        .run();
}

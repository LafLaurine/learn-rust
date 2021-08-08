use bevy::prelude::*;

const PLAYER_SPRITE: &str = "shipA.png";

//create Bevy app

//Commands to spawn/despawn entities, add/remove components on existing entities, manage resources
//Res and ResMut pointers provide read and write access to resources
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>, mut windows: ResMut<Windows>) {
    //add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //get window
    let mut window = windows.get_primary_mut().unwrap();

    //put the sprite at bottom
    let bottom = -window.height()/2.;

    //spawn sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        transform: Transform {
            translation: Vec3::new(0.0, bottom + 20.0, 10.),
            scale: Vec3::new(0.1,0.1,1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn main() {
    App::build()
    //create background color
    .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
    //create window instance
    .insert_resource(WindowDescriptor {
        title:"Invaders".to_string(),
        width: 480.0,
        height: 720.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .run();
}
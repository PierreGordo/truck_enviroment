//For basic usage
use bevy::prelude::*;
//For mouse zooming stuff
use bevy::input::mouse::*;

//Component camera
#[derive(Component)]
struct Camera;



//Component player
#[derive(Component)]
struct Player;

fn main() {
    App::new()
    //This fixes weird white edges around sprites
    .add_plugins(DefaultPlugins
        //This part fixes weird white edges when importing sprite images
        .set(ImagePlugin::default_nearest()))
    .add_systems(Startup, (setup, render_platform, render_background))
    .add_systems(Update, (camera_player_lock, move_player, zoom_camera))
    .run();
}

fn setup(mut commands: Commands){

    //Spawn camera so the stuff actually shows up
    commands.spawn((Camera2d::default(), 
                    Camera));
}

fn render_platform(mut commands: Commands, asset_server: Res<AssetServer>){

    commands.spawn((Sprite::from_image(asset_server.load("military_truck_above.png")),
                    //Doing a simple mark so I can query this sprite later as my player
                    Player));
}

//Rendering background
fn render_background(mut commands: Commands){
    //At first a solid colour, then an image
    commands.spawn((Sprite::from_color ( 
                    Color::srgb(0.0, 0.8, 0.0),
                    Vec2::new(2000.0, 2000.0),
                     ),
                    //Z here is used for ordering (so this will be above elements with 0 z etc..)
                    Transform::from_xyz(0.0, 0.0, 0.0)
                    ));
}


//System to move the camera slowly to test camera movement
//For moving i need to be able to modify the transform, i need time delta and speed
fn camera_player_lock(mut camera_position: Query<&mut Transform, With<Camera>>, mut player_position: Query<&mut Transform, (With<Player>, Without<Camera>)>){

    //I have now queried the component, but I need to query the individual sub components
    for mut camera_transform in &mut camera_position{
       for player_transform in &mut player_position{
            camera_transform.translation = player_transform.translation
       }
    }
}


//Move the player
fn move_player(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut player_position: Query<&mut Transform, With<Player>>){

    //Speed of movement
    let speed = 500.0;
    let rot_speed = 1.0;

    for mut transform in &mut player_position{
        //Defining forward so I can account for rotation when moving
        let forward = transform.up();


        if keys.pressed(KeyCode::KeyW){
            transform.translation += forward * speed * time.delta_secs();
            //Rotation for "turning the truck"
            if keys.pressed(KeyCode::KeyA){
                transform.rotate_z(rot_speed * time.delta_secs());
            }
            if keys.pressed(KeyCode::KeyD){
                transform.rotate_z(-rot_speed * time.delta_secs());
            }

        }
        if keys.pressed(KeyCode::KeyS){
            transform.translation -= forward * speed * time.delta_secs();
            //Rotation for "turning the truck" - just reversed cuz reversing DUH
            if keys.pressed(KeyCode::KeyA){
                transform.rotate_z(-rot_speed * time.delta_secs());
            }
            if keys.pressed(KeyCode::KeyD){
                transform.rotate_z(rot_speed * time.delta_secs());
            }
        }
    }
}

// The "Single" in the query says that there is only one element that matches these parameters so I don't have to for loop later
fn zoom_camera(mouse_wheel: Res<AccumulatedMouseScroll>, camera_query: Single<&mut Projection, With<Camera>>){

    //Since projection is an enum of Perspective, orthographic and custom this match gets out only the orthographic element that i want to modify
    match *camera_query.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            //Adding to scale zooms out (the -= is there since it is reversed -> adding to scale zooms out)
            orthographic.scale -= mouse_wheel.delta.y;
        }
        _ => (),
    }

}
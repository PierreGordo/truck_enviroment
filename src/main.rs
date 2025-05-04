use bevy::color::palettes::css::*;
use bevy::color::palettes::tailwind::*;
//For basic usage
use bevy::prelude::*;
//For mouse zooming stuff
use bevy::input::mouse::*;

//Component camera
#[derive(Component)]
struct Camera;

//Component truck
#[derive(Component)]
struct Truck{
    truck_id: i32,
}

//Component for different points
#[derive(Component)]
struct Point {
    point_id: i32,
}

//Resource to be able to dynamically spawn points
#[derive(Resource, Default)]
struct PointParameters{
    x: f32,
    y: f32,
    id: i32,
}


//Resource for dynamic road drawing
#[derive(Resource, Default)]
struct RoadParameters {
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
    thickness: f32,
    color: Color,
}

//Component for cargo (this will be used later)
#[derive(Component)]
struct Cargo{
    cargo_id: i32,
    //The id of the point where the cargo currently is
    current_point_id: i32,
    //The cargo of the point where the cargo is supposed to be
    target_point_id: i32,
}

fn main() {
    App::new()
    .init_resource::<PointParameters>()
    .init_resource::<RoadParameters>()
    //This fixes weird white edges around sprites
    .add_plugins(DefaultPlugins
        //This part fixes weird white edges when importing sprite images
        .set(ImagePlugin::default_nearest()))
    .add_systems(Startup, (setup, spawn_truck, spawn_point))
    .add_systems(Update, (camera_truck_lock, move_truck, zoom_camera))
    .run();
}

fn setup(mut commands: Commands){

    //Spawn camera so the stuff actually shows up
    commands.spawn((Camera2d::default(), 
                    Camera));
    //Render background
    //At first a solid colour, then an image
    commands.spawn((Sprite::from_color ( 
        Color::from(GREEN_700),
        Vec2::new(2000.0, 2000.0),
         ),
        //Z here is used for ordering (so this will be above elements with 0 z etc..)
        Transform::from_xyz(0.0, 0.0, 0.0)
        ));
}

fn spawn_truck(mut commands: Commands, asset_server: Res<AssetServer>){

    //I will do stuff with the truck id later, so I just dont want to hardcode it into the spawn function
    let truck_id = 1;

    commands.spawn((Sprite::from_image(asset_server.load("military_truck_above.png")),
                    //Using the transform only to change the z-ordering, since it sometimes becomes goofy and gets covered up by the
                    Transform::from_xyz(0.0, 0.0, 1.0),
                    //Doing a simple mark so I can query this sprite later as my player
                    Truck{truck_id: truck_id},
                ));
}

//Function to visualise a point on given coords
//A point is a text and a little circle to signify its position
fn spawn_point(mut commands: Commands, mut mesh: ResMut<Assets<Mesh>>, mut material: ResMut<Assets<ColorMaterial>>, mut parameters: ResMut<PointParameters>) {
    
    //Later I can modify these values when I want to spawn multiple points (even though I wasted my time a little bit setting all of this up, because I could have just hardcoded it in, now it is too lato to turn back and atleast I learned about resources :))
    parameters.x = 100.0;
    parameters.y = -50.0;
    parameters.id = 1;
    
    commands.spawn((Mesh2d(mesh.add(Circle::new(30.0))), 
                        MeshMaterial2d(material.add(Color::from(RED))), 
                        Transform::from_xyz(parameters.x, parameters.y, 5.0),
                        Point{point_id: parameters.id},
                        //For some reason I have to use the format! macro and cannot do it in the same way as regular rust
                    ))
                    //Adding a text child
                    .with_child((Text2d::new(format!("Point: {}", parameters.id)),
                                        //Just bump up the font size and leave the rest default
                                        TextFont{font_size: 50.0, ..default()},
                                        Transform::from_xyz(0.0, -50.0, 5.0),
                    ))
                    ;
    //commands.spawn((Text::new(format!("{}", point_id)),
    //                  Transform::from_xyz(400.0, -5.0, 5.0),));

                        
}



//Very rudimentary system to lock the camera onto the truck, will not be used later
fn camera_truck_lock(mut camera_position: Query<&mut Transform, With<Camera>>, mut player_position: Query<&mut Transform, (With<Truck>, Without<Camera>)>){

    //I have now queried the component, but I need to query the individual sub components
    for mut camera_transform in &mut camera_position{
       for player_transform in &mut player_position{
            camera_transform.translation = player_transform.translation
       }
    }
}


//Move the truck
fn move_truck(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut player_position: Query<&mut Transform, With<Truck>>){

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

    let zoom_speed = 0.5;

    //Removing the match since it does not make much sense to have a match when I only need one thing
    if let Projection::Orthographic(ref mut orthographic) = *camera_query.into_inner(){
        
        //Working not smooth variant
        orthographic.scale -= mouse_wheel.delta.y * zoom_speed;

        //Fix goofy behaviour when zooming in too much (prevents scale from going below 1)
        if orthographic.scale < 1.0{
            orthographic.scale = 1.0;
        }
    }
}
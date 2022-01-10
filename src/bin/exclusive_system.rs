use bevy::prelude::*;

#[derive(Component)]
struct MyComponent;
struct MyResource;

/// Exclusive System を使うことで、App 内の World のすべての要素にアクセスできる
/// https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
fn my_exclusive_system(world: &mut World) {
    println!("Here is my exclusive system");
    world.insert_resource(MyResource);
    world.spawn().insert(MyComponent);
}

fn main() {
    App::new()
        .add_system(my_exclusive_system.exclusive_system())
        .run();
}

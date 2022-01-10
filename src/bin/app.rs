use bevy::prelude::*;

/// My Resource
struct MyCounter(usize);

/// My Event
struct MyEvent;

/// Startup System
fn setup() {
    println!("setup");
}

/// System
fn my_system(mut counter: ResMut<MyCounter>) {
    println!("hello count = {}", counter.0);
    counter.0 += 1;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 標準的な Bevy の機能を追加
        .insert_resource(MyCounter(0)) // Global で唯一な Resource を追加
        .add_event::<MyEvent>() // Event を追加
        .add_startup_system(setup) // 初期化時に一度だけ呼ばれる System
        .add_system(my_system) // System を追加
        .run();
}

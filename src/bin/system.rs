use bevy::prelude::*;

fn setup() {
    println!("Hello from setup");
}

fn hello_world() {
    println!("Hello from system");
}

fn main() {
    App::new()
        .add_startup_system(setup) // 起動時に一度だけ呼ばれる
        .add_system(hello_world) // 毎フレームよばれる
        .run();
}

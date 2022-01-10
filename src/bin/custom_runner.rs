use bevy::prelude::*;

/// App を手動で Update する Runner をカスタマイズできる
fn my_runner(mut app: App) {
    println!("my_runner!");
    app.update();
}

fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    App::new()
        // Custom Runner を適用する
        .set_runner(my_runner)
        .add_system(hello_world)
        .run();
}

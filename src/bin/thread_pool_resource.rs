use bevy::prelude::*;

/// スレッドプールの数を指定することができる
fn main() {
    App::new()
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(4))
        .add_plugins(DefaultPlugins)
        .run();
}

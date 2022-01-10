use bevy::core::FixedTimestep; // 要 import
use bevy::prelude::*;

fn hello_world() {
    println!("hello world");
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_system_set(
            SystemSet::new()
                // FixedTimestep を Run Criteria として設定する
                // Time Step (== Frame Duration) を 0.5 に設定 (2 FPS)
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(hello_world),
        )
        .run();
}

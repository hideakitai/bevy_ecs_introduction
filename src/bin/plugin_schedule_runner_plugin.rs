use bevy::app::{ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy::prelude::*;
use std::time::Duration;

fn hello_world() {
    println!("Hello, world!")
}

fn main() {
    App::new()
        // 1 秒ごとに System が実行されるように設定し、Plugin　を導入
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0,
        )))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_system(hello_world)
        .run();
}

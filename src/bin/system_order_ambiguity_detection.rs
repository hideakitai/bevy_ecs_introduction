// LogPlugin と ReportExecutionOrderAmbiguities を入れる
use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::log::LogPlugin;
use bevy::prelude::*;

fn add(mut counter: ResMut<usize>) {
    *counter += 1;
}

fn print(counter: Res<usize>) {
    if *counter % 2 == 0 {
        println!("{}", *counter);
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // LogPlugin と ReportExecutionOrderAmbiguities を入れると
        // System Order Ambiguity を検出してくれる
        .add_plugin(LogPlugin::default())
        .insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(0_usize)
        .add_system(add)
        .add_system(print)
        .run();
}

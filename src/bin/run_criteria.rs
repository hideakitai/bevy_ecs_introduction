use bevy::ecs::schedule::ShouldRun; // Run Criteria を使用するには追加が必要
use bevy::prelude::*;

/// count が 100 より大きくなったときのみ ShouldRun::Yes を返す Run Criteria
fn my_run_criteria(mut count: Local<usize>) -> ShouldRun {
    if *count > 100 {
        *count = 0;
        ShouldRun::Yes
    } else {
        *count += 1;
        ShouldRun::No
    }
}

/// my_run_criteria を適用する System
fn my_system() {
    println!("Hello, run criteria!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // with_run_criteria() で Run Criteria を System に適用する
        .add_system(my_system.with_run_criteria(my_run_criteria))
        .run();
}

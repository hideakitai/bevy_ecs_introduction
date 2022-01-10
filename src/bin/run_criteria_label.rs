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

/// my_run_criteria を Label で共有する System Set の System
fn my_system_a() {
    println!("Hello, shared run criteria! A");
}

/// my_run_criteria を Label で共有する System Set の System
fn my_system_b() {
    println!("Hello, shared run criteria! B");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(
            my_system
                // my_run_criteria に Label をつけておく
                .with_run_criteria(my_run_criteria.label("MyRunCriteria")),
        )
        // with_run_criteria に Label を指定すれば、その結果が使い回される
        .add_system_set(
            SystemSet::new()
                .with_run_criteria("MyRunCriteria")
                .with_system(my_system_a)
                .with_system(my_system_b),
        )
        .run();
}

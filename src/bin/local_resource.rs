use bevy::prelude::*;

/// System
fn count_up_1() -> impl FnMut() {
    let mut counter = 0usize;
    move || {
        if counter < 10 {
            println!("count_up_2: {}", counter);
            counter += 2;
        }
    }
}
/// counter は count_up_1 とは別のインスタンスになるため、
/// カウントは独立した別のものになる
fn count_up_2(mut counter: usize) -> impl FnMut() {
    move || {
        if counter < 10 {
            println!("count_up_2: {}", counter);
            counter += 2;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(count_up_1())
        .add_system(count_up_2(0))
        .run();
}

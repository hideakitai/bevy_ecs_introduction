use bevy::prelude::*;

/// Local<T> には Default の実装が必要
#[derive(Default)]
struct MyCounter(usize);

/// System
fn count_up_1(mut counter: Local<MyCounter>) {
    if counter.0 < 10 {
        println!("count_up_1: {}", counter.0);
        counter.0 += 1;
    }
}

/// counter は count_up_1 とは別のインスタンスになるため、
/// カウントは独立した別のものになる
fn count_up_2(mut counter: Local<MyCounter>) {
    if counter.0 < 10 {
        println!("count_up_2: {}", counter.0);
        counter.0 += 2;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Default で自動的に Local<MyCounter> が初期化される
        .add_system(count_up_1)
        // .config() を使って手動で Local<MyCounter> を初期化する
        .add_system(count_up_2.config(|params| {
            // 0 番目の引数の Local<MyCounter> を初期化したいので .0 を指定する
            params.0 = Some(MyCounter(0));
        }))
        .run();
}

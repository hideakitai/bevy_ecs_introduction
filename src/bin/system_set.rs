use bevy::prelude::*;

fn first() {
    println!("first");
}

fn second() {
    println!("second");
}

fn third() {
    println!("third");
}

fn fourth() {
    println!("fourth");
}

fn main() {
    App::new()
        // second と third をまとめて SystemSet にする
        .add_system_set(
            SystemSet::new()
                // label をつけておく
                .label("second and third")
                // first よりも後に second と third を実行する
                .after("first")
                // ここにも label をつけておく
                .with_system(second.label("second"))
                // third は second より後に
                .with_system(third.after("second")),
        )
        // first は second and third より前に
        .add_system(first.label("first").before("second and third"))
        // fourth は second and third より後に
        .add_system(fourth.after("second and third"))
        .run();
}

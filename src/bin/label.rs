use bevy::prelude::*;

/// 独自の Label 型を定義する
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
struct MySecondSystemLabel;

fn first() {
    println!("first");
}

fn second() {
    println!("second");
}

fn third() {
    println!("third");
}

fn main() {
    App::new()
        // 独自の Label 型は文字列の Label と同様に使用できる
        .add_system(second.label(MySecondSystemLabel))
        .add_system(first.before(MySecondSystemLabel))
        .add_system(third.after(MySecondSystemLabel))
        .run();
}

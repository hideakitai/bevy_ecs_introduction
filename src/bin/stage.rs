use bevy::prelude::*;

/// 独自の Stage Label を定義する
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
struct MyStage;

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
        // 独自の Stage を Label をつけて登録する (SystemStage::parallel() も使用可能)
        .add_stage_after(CoreStage::Update, MyStage, SystemStage::single_threaded())
        // CoreStage::Update で実行される System を登録
        .add_system(first)
        // MyStage に System を登録
        .add_system_to_stage(MyStage, second)
        .add_system_to_stage(MyStage, third)
        .run();
}

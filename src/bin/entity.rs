use bevy::prelude::*;

/// Components
/// Component を derive した struct や enum が Component として使用できる
#[derive(Component)]

struct Person;
#[derive(Component)]
struct Name(String);

/// Startup System
/// Rust の通常の関数が System として使用できる
/// App の開始時に一度だけ呼ばれる
/// Commands を使い、Person と Name を持った Entity を生成する
fn add_entity(mut commands: Commands) {
    let entity = commands
        .spawn() // Entity の生成
        .insert(Person) // Person Component の追加
        .insert(Name("Bevy".to_string())) // Name Component の追加
        .id(); // Entity を取得

    println!("Entity ID is {}", entity.id());
}

/// App Builder に System を登録し、Bevy の App を構築して Run する
fn main() {
    App::new().add_startup_system(add_entity).run();
}

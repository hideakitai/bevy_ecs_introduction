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
fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Rust".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Bevy".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Ferris".to_string()));
}

/// System
/// Rust の通常の関数が System として使用できる
/// Query を使って Component を取得し、それに対して処理を行う
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

/// App に System を登録し、Bevy の App を構築して Run する
fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(greet_people)
        .run();
}

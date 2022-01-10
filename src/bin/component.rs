use bevy::prelude::*;

// `Component` を実装した struct や enum が Component として使用可能
#[allow(dead_code)]
#[derive(Component, Default)]
struct Position {
    x: f32,
    y: f32,
}

// New Type を使って単純な String を Component として使える
#[derive(Component)]
struct PlayerName(String);

// 空の struct は Marker Component としても使える
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;

fn add_entities(mut commands: Commands) {
    // Player Entity を生成する
    commands
        .spawn() // Entity を生成
        .insert(Player) // Player の Marker Component を追加
        .insert(Position::default()) // Position Component を追加
        .insert(PlayerName("Ferris".to_string())); // PlayerName を追加

    // Enemy Entity を生成する
    commands
        .spawn() // Entity を生成
        .insert(Enemy) // Enemy の Marker Component を追加
        .insert(Position::default()); // Position Component を追加
}

fn main() {
    App::new().add_startup_system(add_entities).run();
}

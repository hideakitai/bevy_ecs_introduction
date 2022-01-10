use bevy::prelude::*;

/// 様々な種類の Component を定義しておく
#[allow(dead_code)]
#[derive(Default, Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Default, Component)]
struct PlayerName(String);
#[derive(Default, Component)]
struct PlayerHp(u32);
#[derive(Default, Component)]
struct PlayerMp(u32);
#[derive(Default, Component)]
struct PlayerXp(u32);

/// Marker Component を定義しておく
#[derive(Default, Component)]
struct Player;
#[derive(Default, Component)]
struct Enemy;

/// 上記の Component を Bundle としてまとめて定義する
/// Bundle を定義するには derive(Bundle) が必要
#[derive(Default, Bundle)]
struct PlayerStatus {
    hp: PlayerHp,
    mp: PlayerMp,
    xp: PlayerXp,
}

/// Bundle は入れ子にすることもできる
#[derive(Default, Bundle)]
struct PlayerBundle {
    name: PlayerName,
    position: Position,
    _marker: Player,
    // Bundle を入れ子にするには #[bundle] が必要
    #[bundle]
    status: PlayerStatus,
}

fn add_entities(mut commands: Commands) {
    // Player Bundle を持った Entity を生成する
    commands.spawn_bundle(PlayerBundle::default());

    // Enemy の Entity を生成する
    // タプルで Component をまとめると Bundle と解釈される
    commands.spawn_bundle((Enemy, Position { x: -1.0, y: -2.0 }));
}

fn main() {
    App::new().add_startup_system(add_entities).run();
}

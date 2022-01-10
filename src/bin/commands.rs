use bevy::prelude::*;

#[derive(Component)]
struct CompA;
#[derive(Component)]
struct CompB;
#[derive(Component)]
struct CompC;

struct MyResource;

fn setup(mut commands: Commands) {
    // Entity の生成と Component/Bundle の追加
    let a = commands.spawn().insert(CompA).id(); // Entity を生成して CompA を追加
    let ab = commands.spawn_bundle((CompA, CompB)).id(); // Bundle を持った Entity を生成
    let abc = commands
        .spawn() // Entity (EntityCommands) を生成
        .insert(CompA) // Component を追加
        .insert_bundle((CompB, CompC)) // Bundle を追加 (タプルは Bundle になる)
        .id(); // id() で Entity を取得できる

    // spawn_batch() で Bundle へのイテレータから複数の Entity を一度に生成できる
    commands.spawn_batch(vec![
        (CompA, CompB, CompC),
        (CompA, CompB, CompC),
        (CompA, CompB, CompC),
    ]);

    // Entity の Component を削除
    commands.entity(abc).remove::<CompB>();

    // Entity の削除
    commands.entity(a).despawn();
    commands.entity(ab).despawn();
    commands.entity(abc).despawn();

    // Resource の追加と削除
    commands.insert_resource(MyResource);
    commands.remove_resource::<MyResource>();
}

fn main() {
    App::new().add_startup_system(setup).run();
}

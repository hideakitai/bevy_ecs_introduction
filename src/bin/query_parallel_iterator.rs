use bevy::prelude::*;
use bevy::tasks::ComputeTaskPool; // 要 import

#[derive(Component)]
struct MyComp(usize);

fn setup(mut commands: Commands) {
    for _ in 0..1000 {
        commands.spawn().insert(MyComp(0));
    }
}

/// parallel_iterator を使用するには、ComputeTaskPool が必要
fn add_one(pool: Res<ComputeTaskPool>, mut query: Query<&mut MyComp>) {
    // parallel_iterator はオーバーヘッドが大きい
    // 下記のような軽い処理や batch size が小さすぎる場合には遅くなる可能性もある
    // プロファイリングを行って、使用すべきかを検討すべき
    const BATCH_SIZE: usize = 100;
    query.par_for_each_mut(&pool, BATCH_SIZE, |mut my_comp| {
        my_comp.0 += 1;
    });
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(add_one)
        .run();
}

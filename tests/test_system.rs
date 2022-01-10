//! cargo test を使って、System のテストを実行することが可能
//! cargo test --test test_system

use bevy::prelude::*;

#[derive(Component)]
struct Counter(usize);

/// テストされる System
fn count_up(mut query: Query<&mut Counter>) {
    for mut counter in query.iter_mut() {
        counter.0 += 1;
    }
}

#[test]
fn has_counted_up() {
    // World を自分で作り、Counter を持った Entity を生成する
    let mut world = World::default();
    let entity = world.spawn().insert(Counter(0)).id();

    // Stage を自分で作り、そこにテストすべき System を追加して実行する
    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(count_up);
    update_stage.run(&mut world);

    // Component を取得して結果をテストする
    let count = world.get::<Counter>(entity).unwrap();
    assert_eq!(count.0, 1);
}

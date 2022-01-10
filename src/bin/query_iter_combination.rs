use bevy::prelude::*;

#[derive(Component)]
struct MyComp(usize);

fn setup(mut commands: Commands) {
    commands.spawn().insert(MyComp(0));
    commands.spawn().insert(MyComp(1));
    commands.spawn().insert(MyComp(2));
    commands.spawn().insert(MyComp(3));
    // commands.spawn().insert(MyComp(4));
    // commands.spawn().insert(MyComp(5));
}

/// 下記を使用すれば、その Query の複数個の組み合わせを総当たりできる
/// - `iter_combinations()` `iter_combinations_mut()` でイテレータを生成
/// - `iter.fetch_next()` で次の組を取得する (`Option`)
fn add_two_comp(query: Query<&MyComp>) {
    let mut iter = query.iter_combinations();
    while let Some([MyComp(c1), MyComp(c2)]) = iter.fetch_next() {
        println!("{} + {} = {}", c1, c2, c1 + c2);
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(add_two_comp)
        .run();
}

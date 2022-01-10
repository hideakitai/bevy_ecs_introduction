use bevy::prelude::*;

/// Default で初期化される Resource
#[derive(Default)]
struct MyFirstCounter(usize);

/// FromWorld で初期化される Resource
struct MySecondCounter(usize);

impl FromWorld for MySecondCounter {
    // ここでは ECS World のすべての要素にアクセスすることが可能
    fn from_world(world: &mut World) -> Self {
        let count = world.get_resource::<MyFirstCounter>().unwrap();
        MySecondCounter(count.0) // MyFirstCounter の値で初期化
    }
}

/// 初期化できない Resource
struct MyThreshold(usize);

/// System
fn count_up(
    mut commands: Commands, // Commands で Resource を追加･削除できる
    mut first_counter: ResMut<MyFirstCounter>, // ミュータブルに参照
    second_counter: Option<ResMut<MySecondCounter>>, // 存在するかわからない Resource
    thresh: Res<MyThreshold>, // イミュータブルに参照
) {
    if let Some(mut second_counter) = second_counter {
        println!("first counter = {}", first_counter.0);
        println!("second counter = {}", second_counter.0);

        first_counter.0 += 1;

        if first_counter.0 % thresh.0 == 0 {
            second_counter.0 += 1;
        }

        if second_counter.0 > 5 {
            commands.remove_resource::<MySecondCounter>();
            println!("second counter has removed");
        }
    } else {
        // do nothing
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 標準的な Bevy の機能を追加
        .init_resource::<MyFirstCounter>() // Default で初期化されて追加
        .init_resource::<MySecondCounter>() // FromWorld で初期化されて追加
        .insert_resource(MyThreshold(5)) // 手動で初期化して追加
        .add_system(count_up) // System を追加
        .run();
}

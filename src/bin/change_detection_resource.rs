use bevy::prelude::*;

/// この Resource の追加と変更を検知する
#[derive(Component)]
struct Counter(usize);

fn count_and_add(
    counter: Option<ResMut<Counter>>, // 存在するか不明なので Option
    mut commands: Commands,
    mut frame_count: Local<usize>,
) {
    if let Some(mut counter) = counter {
        // もし Resource が存在しているならカウントアップする

        if *frame_count < 20 {
            // Counter Resource をインクリメント
            counter.0 += 1;

            *frame_count += 1;

            // 4 フレームごとに Counter Entity をひとつ追加
            if *frame_count % 4 == 0 {
                commands.spawn().insert(Counter(0));
            }
        }
    } else {
        // もし Resource が存在していない場合は追加する
        commands.insert_resource(Counter(0));
    }
}

/// 存在するかわからない Resource の追加と変更を検知する
fn print_added_changed(counter: Option<Res<Counter>>) {
    if let Some(counter) = counter {
        // Counter が追加されたときに実行される
        if counter.is_added() {
            println!("Counter has added");
        }
        // Counter が変更されたときに実行される
        if counter.is_changed() {
            println!("Counter has change to {}", counter.0);
        }
    } else {
        println!("Counter has not found");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(count_and_add.label("CountAndAdd"))
        .add_system(print_added_changed.after("CountAndAdd"))
        .run();
}

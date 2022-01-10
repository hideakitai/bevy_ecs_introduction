use bevy::prelude::*;

/// この Component の追加と変更を検知する
#[derive(Component)]
struct Counter(usize);

fn count_and_add(
    mut query: Query<&mut Counter>,
    mut commands: Commands,
    mut frame_count: Local<usize>,
) {
    if *frame_count < 20 {
        // すべての Counter Component をインクリメント
        for mut counter in query.iter_mut() {
            counter.0 += 1;
        }

        *frame_count += 1;

        // 4 フレームごとに Counter Entity をひとつ追加
        if *frame_count % 4 == 0 {
            commands.spawn().insert(Counter(0));
        }
    }
}

/// ChangeTrackers<T> を Query することで、Component をフィルタリングせずに追加･変更を検知できる
fn print_tracker(query: Query<(Entity, &Counter, ChangeTrackers<Counter>)>) {
    for (entity, counter, trackers) in query.iter() {
        if trackers.is_added() {
            println!("Tracker detected addition {}, {}", entity.id(), counter.0);
        }
        if trackers.is_changed() {
            println!("Tracker detected change {} to {}", entity.id(), counter.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(count_and_add.label("CountAndAdd"))
        .add_system(print_tracker.after("CountAndAdd"))
        .run();
}

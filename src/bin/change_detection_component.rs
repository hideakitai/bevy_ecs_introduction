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

/// Added<T> を Query Filter として、追加された Component を Query
fn print_added(query: Query<(Entity, &Counter), Added<Counter>>) {
    for (entity, counter) in query.iter() {
        println!("Added counter {}, count = {}", entity.id(), counter.0);
    }
}

/// Changed<T> を Query Filter として、変更された Component を Query
fn print_changed(query: Query<(Entity, &Counter), Changed<Counter>>) {
    for (entity, counter) in query.iter() {
        println!("Changed counter entity {} to {}", entity.id(), counter.0);
    }
}

// By looking at trackers, the query is not filtered but the information is available
fn print_tracker(query: Query<(Entity, &Counter, ChangeTrackers<Counter>)>) {
    for (entity, component, trackers) in query.iter() {
        if trackers.is_added() {
            println!("Tracker detected addition {}, {}", entity.id(), component.0);
        }
        if trackers.is_changed() {
            println!("Tracker detected change {} to {}", entity.id(), component.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(count_and_add.label("CountAndAdd"))
        .add_system(print_added.after("CountAndAdd"))
        .add_system(print_changed.after("CountAndAdd"))
        .add_system(print_tracker.after("CountAndAdd"))
        .run();
}

use bevy::prelude::*;

#[derive(Component)]
struct MyTimer(Timer);

fn setup(mut commands: Commands) {
    // MyTimer を 2.0 秒ごとに繰り返すように設定
    commands
        .spawn()
        .insert(MyTimer(Timer::from_seconds(2.0, true)));
}

// MyTimer に経過時間を Time Resource を使って適用し、指定時間が経過したかをチェックする
fn print_timer(time: Res<Time>, mut query: Query<&mut MyTimer>) {
    let mut my_timer = query.single_mut();
    if my_timer.0.tick(time.delta()).just_finished() {
        println!("Tick");
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(setup)
        .add_system(print_timer)
        .run();
}

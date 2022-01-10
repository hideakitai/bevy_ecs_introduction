use bevy::prelude::*;

/// State は enum として定義する
/// Debug, Clone, PartialEq, Eq, Hash を実装する必要がある
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Game,
    Paused,
}

/// AppState::Game の動作中に毎フレーム呼ばれる
fn game_on_update(mut count: Local<usize>, mut app_state: ResMut<State<AppState>>) {
    if *count < 6 {
        println!("Game on_update() frame = {}", *count);
        *count += 1;

        if *count == 3 {
            println!("Game -> Paused with state stack push()");
            app_state.push(AppState::Paused).unwrap();
        } else if *count == 6 {
            println!("finish");
        }
    }
}

/// AppState::Game が Inactive になる際に一度だけ呼ばれる
fn game_on_pause() {
    println!("Game on_pause()");
}

/// AppState::Game が Inactive でも Active でも毎フレーム呼ばれる
/// ただし Bevy 0.5.0 にはバグがあり、on_update() と同じ動作しかしてくれない
/// https://github.com/bevyengine/bevy/issues/3179
fn game_on_in_stack_update(mut count: Local<usize>) {
    if *count < 6 {
        println!("Game on_in_stack_update() frame = {}", *count);
        *count += 1;
    }
}

/// AppState::Game が Inactive なときだけ毎フレーム呼ばれる
fn game_on_inactive_update(mut count: Local<usize>) {
    println!("Game on_inactive_update() frame = {}", *count);
    *count += 1;
}

/// AppState::Game が Active に戻る際に一度だけ呼ばれる
fn game_on_resume() {
    println!("Game on_resume()");
}

/// AppState::Paused の開始時に一度だけ呼ばれる
fn paused_on_enter() {
    println!("Pause on_enter()");
}

/// AppState::Paused の動作中に毎フレーム呼ばれる
fn paused_on_update(mut count: Local<usize>, mut app_state: ResMut<State<AppState>>) {
    println!("Pause on_update() frame = {}", *count);
    *count += 1;

    if *count >= 3 {
        println!("Paused -> Game with state stack pop()");
        app_state.pop().unwrap();
    }
}

/// AppState::Paused の終了時 (遷移時) に一度だけ呼ばれる
fn paused_on_exit() {
    println!("Paused on_exit()");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Game)
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_on_update))
        // AppState::Game が Inactive になる際に一度だけ呼ばれる
        .add_system_set(SystemSet::on_pause(AppState::Game).with_system(game_on_pause))
        // AppState::Game が Inactive でも Active でも毎フレーム呼ばれる
        // ただし Bevy 0.5.0 にはバグがあり、on_update() と同じ動作しかしてくれない
        // https://github.com/bevyengine/bevy/issues/3179
        .add_system_set(
            SystemSet::on_in_stack_update(AppState::Game).with_system(game_on_in_stack_update),
        )
        // AppState::Game が Inactive なときだけ毎フレーム呼ばれる
        .add_system_set(
            SystemSet::on_inactive_update(AppState::Game).with_system(game_on_inactive_update),
        )
        // AppState::Game が Active に戻る際に一度だけ呼ばれる
        .add_system_set(SystemSet::on_resume(AppState::Game).with_system(game_on_resume))
        .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(paused_on_enter))
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(paused_on_update))
        .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(paused_on_exit))
        .run();
}

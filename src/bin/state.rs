use bevy::prelude::*;

/// State は enum として定義する
/// Debug, Clone, PartialEq, Eq, Hash を実装する必要がある
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Menu,
    Game,
    End,
}

/// State に関係なく常に動作する System
/// System 内で State にアクセスするためには `State<T>` Resource を使う
fn count_frame(mut count: Local<usize>, app_state: Res<State<AppState>>) {
    // current() で現在の State が取得できる
    if *app_state.current() != AppState::End {
        println!("global frame = {}", *count);
        *count += 1;

        match app_state.current() {
            AppState::Menu => println!("state is Menu"),
            AppState::Game => println!("state is Game"),
            _ => (),
        }
    }
}

/// AppState::Menu に Enter したときのみ動作する System
/// System 内で State にアクセスするためには `State<T>` Resource を使う
fn menu_on_enter(mut app_state: ResMut<State<AppState>>) {
    println!("entered to menu state");

    // Menu から Game へ変更するが、下記の場合に失敗する可能性がある
    // - すでにその State になっている場合
    // - 他の State への変更がすでにキューに積まれている場合
    app_state.set(AppState::Game).unwrap(); // すでにその State の場合などに失敗する
}

/// AppState::Menu を Exit したときのみ動作する System
fn menu_on_exit() {
    println!("exit menu state");
}

/// AppState::Game に Enter したときのみ動作する System
fn game_on_enter() {
    println!("entered to game state");
}

/// AppState::Game の Update 時にだけ動作する System
fn game_on_update(mut count: Local<usize>, mut app_state: ResMut<State<AppState>>) {
    println!("game frame = {}", *count);
    *count += 1;

    if *count >= 5 {
        app_state.set(AppState::End).unwrap();
    }
}

/// AppState::Game を Exit したときのみ動作する System
fn game_on_exit() {
    println!("exit game state");
}

/// AppState::End へ Enter したときのみ動作する System
fn end_on_enter() {
    println!("entered to end state, finish app");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // State の初期値を登録して State を有効化する
        .add_state(AppState::Menu)
        // State に関係なく動作する System
        .add_system(count_frame)
        // AppState::Menu のときのみ動作する System Set
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(menu_on_enter))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(menu_on_exit))
        // AppState::Game のときのみ動作する System Set
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game_on_enter))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_on_update))
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(game_on_exit))
        // AppState::End のときのみ動作する System Set
        .add_system_set(SystemSet::on_enter(AppState::End).with_system(end_on_enter))
        .run();
}

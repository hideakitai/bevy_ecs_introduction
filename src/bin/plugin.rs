use bevy::prelude::*;

/// Plugin にまとめたい様々な要素
struct MyResource;
struct MyEvent;

/// 自作の Plugin
struct MyPlugin;

/// 自作の Plugin に Plugin トレイトを実装すれば、Plugin として使用できる
/// Plugin トレイトでは App Builder に必要な要素を追加するだけで良い
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyResource)
            .add_event::<MyEvent>()
            .add_startup_system(plugin_setup)
            .add_system(plugin_system);
    }
}

/// Plugin にまとめたい startup_system
fn plugin_setup() {
    println!("MyPlugin's setup");
}

/// Plugin にまとめたい system
fn plugin_system() {
    println!("MyPlugin's system");
}

fn main() {
    App::new()
        .add_plugin(MyPlugin) // Plugin を追加する
        .run();
}

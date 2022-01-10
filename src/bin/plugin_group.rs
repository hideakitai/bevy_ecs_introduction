use bevy::app::PluginGroupBuilder; // PluginGroup トレイトを実装するには追加が必要
use bevy::prelude::*;

/// 自作の Plugin
struct FooPlugin;
struct BarPlugin;

impl Plugin for FooPlugin {
    fn build(&self, _app: &mut App) {
        println!("build FooPlugin");
    }
}

impl Plugin for BarPlugin {
    fn build(&self, _app: &mut App) {
        println!("build BarPlugin");
    }
}

/// 自作の Plugin Group
struct MyPluginGroup;

impl PluginGroup for MyPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(FooPlugin).add(BarPlugin); // Plugin を group に追加
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyPluginGroup) // PluginGroup を追加
        .run();
}

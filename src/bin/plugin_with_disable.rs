use bevy::audio::AudioPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |plugins| {
            plugins
                .disable::<AudioPlugin>() // AudioPlugin を無効化
                .disable::<LogPlugin>() // LogPlugin を無効化
        })
        .run();
}

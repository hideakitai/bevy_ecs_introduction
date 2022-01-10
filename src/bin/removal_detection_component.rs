use bevy::prelude::*;

#[derive(Component)]
struct MyComponent;

/// MyComponent を追加する
fn add_components(mut commands: Commands) {
    for _ in 0..5 {
        let entity = commands.spawn().insert(MyComponent).id();
        println!("MyComponent Entity {} has added", entity.id());
    }
}

/// CoreStage::Update で MyComponent が存在するときは削除する
fn remove_components_if_exist(query: Query<Entity, With<MyComponent>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).remove::<MyComponent>();
    }
}

/// CoreStage::PostUpdate で MyComponent の削除を検知する
/// RemovedComponents<T> でこれまでに削除された Component を検出できる
fn detect_removals(removals: RemovedComponents<MyComponent>) {
    for entity in removals.iter() {
        println!("MyComponent Entity {} has removed", entity.id());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Component を追加する
        .add_startup_system(add_components)
        // CoreStage::Update で MyComponent が存在するときは削除する
        .add_system(remove_components_if_exist)
        // CoreStage::PostUpdate で MyComponent の削除を検知する
        .add_system_to_stage(CoreStage::PostUpdate, detect_removals)
        .run();
}

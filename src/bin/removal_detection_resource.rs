use bevy::prelude::*;

struct MyResource;

/// 3 フレーム経ったら MyResource を削除する
fn remove_resource_after_3_frame(
    mut frame: Local<usize>,
    my_resource: Option<Res<MyResource>>,
    mut commands: Commands,
) {
    if let Some(_) = my_resource {
        *frame += 1;

        if *frame >= 3 {
            commands.remove_resource::<MyResource>();
        }
    }
}

/// 前フレームで Resource が存在したかを保存することで、次のフレームで削除を検知する
fn detect_removals(my_resource: Option<Res<MyResource>>, mut my_resource_existed: Local<bool>) {
    if let Some(_) = my_resource {
        println!("MyResource exists");
        *my_resource_existed = true;
    } else if *my_resource_existed {
        println!("MyResource has removed");
        *my_resource_existed = false;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyResource)
        .add_system(remove_resource_after_3_frame)
        .add_system(detect_removals)
        .run();
}

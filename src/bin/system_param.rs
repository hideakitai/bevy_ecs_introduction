use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component)]
struct MyComponent;
#[derive(Component)]
struct MyResource(usize);

/// SystemParam は System の引数にできるものは何でも持つことができる
/// SystemParam トレイトを derive する必要がある
/// Bevy 0.6.0 時点では Lifetime は下記のように指定する
/// - `SystemParam` を derive する際は `<'w, 's>` と指定する必要がある
///   - `'w` : `World` の Lifetime
///   - `'s` : `State` の Lifetime
/// - Query は `<'w, 's>`, Resource は `<'w>`, Local Resource は `<'s>` を要求する
/// - Query 内の Component の Lifetime は `'static` を要求する
/// - メンバが `'w` `'s` を要求しない場合は `PhantomData` を導入する必要がある
#[derive(SystemParam)]
struct MySystemParam<'w, 's> {
    query: Query<'w, 's, (Entity, &'static MyComponent)>,
    resource: ResMut<'w, MyResource>,
    local: Local<'s, usize>,
}

#[allow(dead_code)]
#[derive(SystemParam)]
struct MySystemParam2<'w, 's> {
    resource: ResMut<'w, MyResource>,

    // 's を満たすために PhantomData が必要
    #[system_param(ignore)]
    _secret: PhantomData<&'s ()>,
}

/// SystemParam を実装する型は、通常通りメソッドを実装できる
impl<'w, 's> MySystemParam<'w, 's> {
    fn my_component_ids(&self) -> Vec<u32> {
        self.query.iter().map(|(e, _)| e.id()).collect::<Vec<_>>()
    }

    fn increment_my_resource(&mut self) {
        self.resource.0 += 1;
    }

    fn my_resource_data(&self) -> usize {
        self.resource.0
    }

    fn increment_my_local_data(&mut self) {
        *self.local += 1;
    }

    fn my_local_data(&self) -> usize {
        *self.local
    }
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(MyComponent);
    commands.spawn().insert(MyComponent);
    commands.spawn().insert(MyComponent);
}

/// SystemParam は直接 System の引数にすることができる
fn query_system_param(mut my_system_param: MySystemParam) {
    println!(
        "my entity component are {:?}",
        my_system_param.my_component_ids()
    );

    my_system_param.increment_my_resource();
    println!("my resource data is {}", my_system_param.my_resource_data());

    my_system_param.increment_my_local_data();
    println!("my local resource is {}", my_system_param.my_local_data());
}

fn main() {
    App::new()
        .insert_resource(MyResource(2))
        .add_startup_system(setup)
        .add_system(query_system_param)
        .run();
}

use bevy::prelude::*;

#[derive(Component)]
struct MyParent(String);
#[derive(Component)]
struct MyChild(String);

fn setup(mut commands: Commands) {
    let parent = commands
        .spawn()
        .insert(MyParent("MyParent".to_string())) // Entity を MyParent とし、
        .with_children(|parent| {
            parent.spawn().insert(MyChild("MyChild1".to_string())); // Child として MyChild を生成
        })
        .id();

    // 別途 MyChild Entity を生成し
    let child = commands
        .spawn()
        .insert(MyChild("MyChild2".to_string()))
        .id();

    // MyParent の Entity ID を使って MyChild を Child として追加
    commands.entity(parent).push_children(&[child]);
}

/// Child Entity の Query から、それぞれの Parent Entity を取得する
///
/// 非常に紛らわしいが、
/// - Child となった Entity には Parent Component が付与される
/// - Parent となった Entity には Children Component が付与される
/// そのため、
/// - Parent Entity を Query するには Query<&Children> を使う
/// - Child Entity を Query するには Query<&Parent> を使う
fn find_parent_from_children(
    q_child: Query<&Parent>, // Parent Component を持つ Child Entity を Query
    q_my_parent: Query<&MyParent>, // MyParent を Query
) {
    println!("===== find parent =====");

    // Child Entity が持っている Parent Component をイテレート
    for parent in q_child.iter() {
        // Parent Component は Entity (ID) を 0 番目の要素として持つので、
        // それを使って q_my_parent から MyParent を取得する
        let p = q_my_parent.get(parent.0);

        if let Ok(p) = p {
            println!("{}", p.0);
        }
    }
}

/// Parent Entity の Query から、Children Entity を取得する
///
/// 非常に紛らわしいが、
/// - Child となった Entity には Parent Component が付与される
/// - Parent となった Entity には Children Component が付与される
/// そのため、
/// - Parent Entity を Query するには Query<&Children> を使う
/// - Child Entity を Query するには Query<&Parent> を使う
fn find_children_from_parent(
    q_parent: Query<&Children>,  // Children Component を持つ Parent を Query
    q_my_child: Query<&MyChild>, // MyChild を Query
) {
    println!("===== find children =====");

    // Parent Entity が持っている Children Component をイテレート
    for children in q_parent.iter() {
        // Children Component は Child Entity ID の集合を持っているのでイテレート
        for &child in children.iter() {
            // Child Entity ID を使って、q_my_child から MyChild を取得する
            let my_child = q_my_child.get(child);

            if let Ok(my_child) = my_child {
                println!("{}", my_child.0);
            }
        }
    }
}

/// MyParent の Entity ID を Query し、それを使って MyParent を MyChild 含めて再帰的に破棄する
/// MyParent は Children を持っているので、それを Query Filter として使う
fn clean_up(mut commands: Commands, query: Query<Entity, With<Children>>) {
    let e = query.single(); // MyParent は唯一のはず
    commands.entity(e).despawn_recursive(); // MyParent を MyChild 含めて再帰的に破棄
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(find_parent_from_children.before("child"))
        .add_system(find_children_from_parent.label("child"))
        .add_system(clean_up.after("child"))
        .run();
}

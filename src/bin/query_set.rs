use bevy::prelude::*;

#[derive(Debug, Component)]
struct CompA(String);
#[derive(Debug, Component)]
struct CompB(String);
#[derive(Debug, Component)]
struct CompC(String);

fn setup(mut commands: Commands) {
    commands.spawn().insert(CompA("A".to_string()));
    commands.spawn_bundle((CompA("AB-A".to_string()), CompB("AB-B".to_string())));
    commands.spawn_bundle((CompA("AC-A".to_string()), CompC("AC-C".to_string())));
    commands.spawn_bundle((
        CompA("ABC-A".to_string()),
        CompB("ABC-B".to_string()),
        CompC("ABC-C".to_string()),
    ));
}

fn my_system(
    mut q: QuerySet<(
        QueryState<&mut CompA, With<CompB>>, // A with B (Mutable)
        QueryState<&mut CompA, With<CompC>>, // A with C (Mutable)
    )>,
) {
    println!("===== Query A with B =====");
    for mut a in q.q0().iter_mut() {
        a.0 += " wb";
        println!("{:?}", a);
    }

    println!("===== Query A with C =====");
    for mut a in q.q1().iter_mut() {
        a.0 += " wc";
        println!("{:?}", a);
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(my_system)
        .run();
}

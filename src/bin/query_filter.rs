use bevy::prelude::*;

#[derive(Debug, Component)]
struct CompA(String);
#[derive(Debug, Component)]
struct CompB(String);
#[derive(Debug, Component)]
struct CompC(String);
#[derive(Debug, Component)]
struct CompD(String);

fn setup(mut commands: Commands) {
    commands.spawn().insert(CompA("A".to_string()));
    commands.spawn().insert(CompB("B".to_string()));
    commands.spawn_bundle((CompA("AC-A".to_string()), CompC("AC-C".to_string())));
    commands.spawn_bundle((CompA("AD-A".to_string()), CompD("AD-D".to_string())));
    commands.spawn_bundle((
        CompA("ACD-A".to_string()),
        CompC("ACD-C".to_string()),
        CompD("ACD-D".to_string()),
    ));
}

fn my_system(
    q_a_wc: Query<&CompA, With<CompC>>,            // A with C
    q_a_woc: Query<&CompA, Without<CompC>>,        // A without C
    q_ac_wd: Query<(&CompA, &CompC), With<CompD>>, // A + C with D
    q_a_wc_wod: Query<&CompA, (With<CompC>, Without<CompD>)>, // A with C && without D
    q_a_wc_wd: Query<&CompA, Or<(With<CompC>, With<CompD>)>>, // A with C || with D
) {
    println!("===== Query A with C =====");
    for a in q_a_wc.iter() {
        println!("{:?}", a);
    }

    println!("===== Query A without C =====");
    for a in q_a_woc.iter() {
        println!("{:?}", a);
    }

    println!("===== Query A + C with D =====");
    for (a, c) in q_ac_wd.iter() {
        println!("{:?}, {:?}", a, c);
    }

    println!("===== Query A with C && without D =====");
    for a in q_a_wc_wod.iter() {
        println!("{:?}", a);
    }

    println!("===== Query A with C || with D =====");
    for a in q_a_wc_wd.iter() {
        println!("{:?}", a);
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(my_system)
        .run();
}

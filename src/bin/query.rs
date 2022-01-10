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
}

fn my_system(
    q_a: Query<&CompA>,                    // A を持つ Entity (例では3つ)
    mut q_b: Query<&mut CompB>,            // B を持つ Entity (例では1つ, Mutable)
    q_ac: Query<(&CompA, &CompC)>,         // A と C を両方持つ Entity
    q_ad: Query<(&CompA, Option<&CompD>)>, // A と持っていれば D
    q_ae: Query<(&CompA, Entity)>,         // A とその Entity ID
) {
    println!("===== Query A =====");
    for a in q_a.iter() {
        println!("{:?}", a);
    }

    println!("===== Query B =====");
    let mut b = q_b.single_mut();
    println!("{:?}", b);
    b.0 += " Mutable";
    println!("{:?}", b);

    println!("===== Query A && C =====");
    for (a, c) in q_ac.iter() {
        println!("{:?}, {:?}", a, c);
    }

    println!("===== Query A && Option<D> =====");
    for (a, d) in q_ad.iter() {
        println!("{:?}, {:?}", a, d);
    }

    println!("===== Query A + Entity =====");
    for (a, e) in q_ae.iter() {
        println!("{:?}, {:?}", a, e); // Entity shows {Xv0}, X: entity id, 0: generation
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(my_system)
        .run();
}

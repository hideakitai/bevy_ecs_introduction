use bevy::prelude::*;

fn first() {
    println!("first");
}

fn second() {
    println!("second");
}

fn third() {
    println!("third");
}

fn fourth() {
    println!("fourth");
}

fn main() {
    App::new()
        // second という label をつける
        .add_system(second.label("second"))
        // first は second より前に
        .add_system(first.before("second"))
        // fourth は second より後に
        .add_system(fourth.label("fourth").after("second"))
        // third は second より後 fourth より前に
        .add_system(third.after("second").before("fourth"))
        .run();
}

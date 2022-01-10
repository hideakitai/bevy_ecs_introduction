use anyhow::Result;
use bevy::prelude::*;

/// 処理の結果を後段の System へ転送する
fn parse_number() -> Result<()> {
    // let s = "3".parse::<i32>()?; // Ok
    let s = "number".parse::<i32>()?; // Err
    println!("parsed number is {}", s);
    Ok(())
}

/// 前段の System から送られてきた Result が Err だったら処理する
fn handle_error(In(result): In<Result<()>>) {
    if let Err(e) = result {
        println!("parse error: {:?}", e);
    }
}

fn main() {
    App::new()
        .add_system(
            parse_number.chain(handle_error), // chain() で後段の System を登録する
        )
        .run();
}

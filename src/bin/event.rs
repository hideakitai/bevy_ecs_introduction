use bevy::prelude::*;

/// 送受信する Event
/// 内部に送受信したい様々なデータをもたせる
struct MyEvent(String);

/// Event を送信する System
/// EventWriter を使って MyEvent を送信する
fn event_write(mut event_writer: EventWriter<MyEvent>) {
    event_writer.send(MyEvent("Hello, event!".to_string()))
}

/// Event を受信する System
/// EventReader のイテレータを使って受信 Event を処理する
fn event_read(mut event_reader: EventReader<MyEvent>) {
    for e in event_reader.iter() {
        let msg = &e.0;
        println!("received event: {}", msg);
    }
}

fn main() {
    App::new()
        .add_event::<MyEvent>() // Event を App に登録する必要がある
        .add_system(event_write)
        .add_system(event_read)
        .run();
}

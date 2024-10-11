// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,           // 添加 Quit 变体
    Echo(String),   // 添加 Echo 变体
    Move { x: i32, y: i32 }, // 添加 Move 变体
    ChangeColor(i32, i32, i32), // 添加 ChangeColor 变体
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("echo message")));
    println!("{:?}", Message::Move { x: 5, y: 6 });
    println!("{:?}", Message::ChangeColor(255,0,0));
}

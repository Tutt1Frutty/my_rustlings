// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello")));
    println!("{:?}", Message::Move{x: 10, y: 20});
    println!("{:?}", Message::ChangeColor(255, 255, 255));
}
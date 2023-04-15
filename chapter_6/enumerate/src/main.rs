// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }

// fn main() {
//     let four = IpAddrKind::V4(127,0,0,1);
//     let six = IpAddrKind::V6(String::from("::1"));

//     println!("four: {:?}", four);
//     println!("six: {:?}", six);
// }

// ---

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main(){
    let m = Message::Write(String::from("hello"));
    m.call();

    println!("m: {:?}", m);
}
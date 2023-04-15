/**
 * 
 * 사각형의 넓이를 계산하는 프로그램
 * 
 * 픽셀 단위로 명시된 사각형의 길이와 너비를 입력받아서 사각형의 넓이를 계산하는 프로그램을 작성한다.
 * 
 */

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle{
//         width: 50,
//         height:30
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


/**
 * 파생 트레잇(derive trait)으로 유용한 기능 추가하기
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 50,
        height:30
    };

    println!("rect1 is {:#?}", rect1);
}
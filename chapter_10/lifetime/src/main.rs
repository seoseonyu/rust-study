fn main() {
    // 아래 코드에서는 변수 x의 라이프타임이 블록 내에서만 유효하므로
    // 블록 밖에서는 참조할 수 없다.

    // let r; // r 생성

    // {
    //     let x = 5; // x 생성
    //     r = &x; // r에 x의 참조를 저장
    // } // x 소멸

    // println!("r: {}", r); // r을 사용: 여기서 r은 이미 소멸한 x를 참조하게 되므로 오류발생

    // 함수에서의 제네릭 라이프타임

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

/**
 * 제네릭 라이프타임
 *
 * 해당함수는 반환 타입에 대하여 제네릭 라이프타임 파라미터가 필요하다는 것을 말해주고 있는데
 * 왜냐하면 반환되는 참조자가 x를 참조하는지 y를 참조하는지 러스트는 알 수 없기때문이다.
 * 이를 해결하기 위해서는 참조자들 간의 관계를 정의하는 제네릭 라이프타임 파라미터를 추가해야한다.
 */
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 2023-04-20 이어서 https://rust-kr.github.io/doc.rust-kr.org/ch10-03-lifetime-syntax.html#%EA%B5%AC%EC%A1%B0%EC%B2%B4-%EC%A0%95%EC%9D%98%EC%97%90%EC%84%9C-%EB%9D%BC%EC%9D%B4%ED%94%84%ED%83%80%EC%9E%84-%EB%AA%85%EC%8B%9C%ED%95%98%EA%B8%B0

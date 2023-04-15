/**
 * struct_ownership
 * 
 * 구조체의 데이터를 정의할때 소유권이 없는 데이터의 참조를 저장할수는 있지만 lifetime을 명시해야 한다.
 * 라이프타임은 구조체가 존재하는동안 참조하는 데이터를 계속 존재할 수 있도록 한다.
 * 라이프타임을 사용하지 않고 참조를 저장하고자 하면 아래와같이 lifetime 에러가 발생한다.
 * 
 */

struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "dev@seoseonyu.com",
        username:"seoseonyu",
        active: true,
        sign_in_count: 1,
    };
    
    println!("Hello, world!");
}

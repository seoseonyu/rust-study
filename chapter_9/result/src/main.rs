use std::io;
use std::io::Read;
use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.text") {
            Ok(fc) => fc,
            Err(e) => panic!("파일을 생성할 수 없습니다: {:?}", e),
        },
        Err(error) => panic!("파일을 열 수 없습니다: {:?}", error),
    };

    // 에러가 났을 때 패닉을 위한 숏컷: unwrap, expect

    // match의 간결 화 unwrap()
    // Result값이 OK일경우 값을 반환하고 Err일 경우 panic!을 호출한다.
    let f = File::open("hello.text").unwrap();

    // expect()는 unwrap()과 비슷하지만 panic!메시지를 지정할 수 있다.
    let f = File::open("hello.text").expect("파일을 열 수 없습니다.");
}

// 에러 전파하기
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.text");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // 여기서 Return 이 없어도 되는 이유는 함수의 마지막 표현식이기 때문이다.
    }
}

// 에러 전파를 위한 숏컷
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.text")?; // ?를 사용하면 Ok()를 받으면 값을 반환하고 Err()를 받으면 Err()와 함께 함수를 즉시 반환한다.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortcut2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.text")?.read_to_string(&mut s)?; // ?를 연속으로 사용할 수 있다.

    Ok(s)
}

// ?를 사용할 수 있는 함수는 Result를 반환하는 함수여야 한다.
// main()함수는 Result를 반환하지 않기 때문에 ?를 사용할 수 없다.

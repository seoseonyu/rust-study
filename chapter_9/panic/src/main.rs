fn main() {
    // panic!("crash and burn")

    // panic! 백트레이스

    let v = vec![1, 2, 3];

    // v[99]; // panic! 발생
    v[99]; // buffer overread

    // 환경변수 RUST_BACKTRACE=1 를 포함하여 실행시킬경우 panic 발생시 백트레이스 출력
}

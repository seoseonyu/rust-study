fn main() {
    // let v: Vec<i32> = Vec::new();

    // let v = vec![1, 2, 3];

    // --- 벡터 갱신하기

    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);


    // --- 백터 요소 읽기

    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);

    // --- 유요하지 않은 참조자
    // ? 왜되지?
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];
    // println!("The first element is: {}", first);
    // v.push(6);
    // println!("Vector: {:?}", v);

    //  ---
    
    // let v= vec![100, 32, 57];

    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut mut_v = vec![100, 32, 57];

    // for i in &mut mut_v {
    //     // *i += 50; 의 (*)는 역참조 
    //     *i += 50;
    // }

    // ---

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

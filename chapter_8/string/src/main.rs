fn main() {
    let mut s = String::new();

    // string literal to String

    /* let data = "initial contents";

       let ss = data.to_string();

       let ss = "initial contents".to_string();
    */


    // ---

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
}

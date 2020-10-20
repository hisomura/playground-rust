fn main() {
    let s1: String = String::from("Hello, World");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);


    let mut s4: String = String::from("Hello, World");
    // let s5: &str = &s4;
    s4.make_ascii_lowercase();
    println!("{}", s4);
}

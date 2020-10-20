fn main() {
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    println!("{:?}", a);
    a[1] =b[1];
    a[2] =b[2];
    // println!("{:?}", a[1..3]); // error
    println!("{:?}", &a[1..3]);
    println!("{:?}", a);
}

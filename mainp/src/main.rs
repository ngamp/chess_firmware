fn main() {
    println!("hello world");
    println!("error before!");
    println!("hallo");
    let mut test_vec = vec![0,1,2,3,4,5,6,7];
    test_vec.drain(2..5);
    println!("{:?}", test_vec)
}


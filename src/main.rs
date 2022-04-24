fn main() {
    println!("Hello, world!");


    use random_string::generate;

    let charset = "1234567890";
    println!("{}", generate(10, charset));
}

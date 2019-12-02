mod ab;

fn main() {
    println!("Hello, world!");
    let result = ab::create_string(11, 30);
    println!("{}", result);
}
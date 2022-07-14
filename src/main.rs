fn main() {
    println!("random: {}", rand::random::<u64>().overflowing_add(1).0);
    println!("Hello, world!");
}

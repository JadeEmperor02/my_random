use my_random::MyRandom;
fn main() {
    // the rand_range method is called with teo values a min and a max to get a random number
    let mut rng: MyRandom = MyRandom::new();
    println!("randomized value {}", rng.rand_range(0, 10));
    println!("success");
}

use my_random::MyRandom;
fn main() {
    // the rand_bool takes no parameter and returns a random boolean
    let mut rng: MyRandom = MyRandom::new();
    for _ in 0..10 {
        println!("randomized boolean {}", rng.rand_bool());
    }
    println!("success");
}

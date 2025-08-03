use my_random::MyRandom;

fn main() {
    // automatically returns a random float when called
    let mut rng: MyRandom = MyRandom::new();
    println!("Random float: {}", rng.rand_float());
    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("Random float: {}", rng.rand_float());
        if count >= 10 {
            break;
        }
    }
    println!("success");
}

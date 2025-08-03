use my_random::MyRandom;
fn main() {
    // the rand_token is called with a parameter which tells it how long the token has to be
    let mut rng: MyRandom = MyRandom::new();
    println!("randomized token {}", rng.rand_token(100));
    println!("success");
}

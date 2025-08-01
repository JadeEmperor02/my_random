use my_random::MyRandom;

fn main() {
    let mut rng: MyRandom = MyRandom::new();
    println!("Random float: {}", rng.rand());
    println!("Random bool: {}", rng.rand_bool());

    let items: Vec<&'static str> = vec!["apple", "banana", "cherry"];
    let sample: Vec<&'static str> = rng.rand_vector(&items, 2);
    println!("Sampled: {:?}", sample);
}

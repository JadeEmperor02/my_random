use my_random::MyRandom;
fn main() {
    // the rand_vector takes to parameter, a reference to a slice '&[T]' and a count to tell it how long the new vector will be
    let mut rng: MyRandom = MyRandom::new();
    let slice: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    // Note: it will never return more than the length of the vector this is to prevent duplicated values
    let shuffled = rng.rand_vector(&slice, 11);
    println!("randomized vector {:?}", shuffled);
    println!("success");
}

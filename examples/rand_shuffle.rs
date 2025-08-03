use my_random::MyRandom;
fn main() {
    // the rand_shuffle method takes a reference to a  mutable slice and shuffles it
    let mut rng: MyRandom = MyRandom::new();
    let mut slice: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let shuffled = rng.rand_shuffle(&mut slice);
    println!("shuffled value {:?}", shuffled);
    println!("success");
}

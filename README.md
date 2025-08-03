# my_random

**my_random** is a simple and concise program that allow you to generate a random number mimicking the `randrange` function in python, by leveraging its clear syntax you can easily generate tokens, shuffled slices, randomized booleans and lots more.

Heres how to use the crate

## Features

- `rand_range()` – Generate a random number between two bounds.
- `rand_shuffle()` – Shuffle a vector or slice.
- `rand_bool()` – Get a random boolean.
- `rand_vector()` – Generate a vector of random numbers.
- `rand_token()` – Create a random alphanumeric token.
- `rand_float()` – Generate a random float.

## 📦 Usage

add the package to your `Cargo toml`:

```
toml
my_random = "0.1.1"

```

## 🧪 Examples

all done check out some use cases.

`Random Number using rand_range`:

```
use my_random::MyRandom;
fn main() {
    // the rand_range method is called with teo values a min and a max to get a random number
    let mut rng: MyRandom = MyRandom::new();
    println!("randomized value {}", rng.rand_range(0, 10));
    println!("success");
}
```

`random token generation using rand_token` :

```
use my_random::MyRandom;
fn main() {
    // the rand_token is called with a parameter which tells it how long the token has to be
    let mut rng: MyRandom = MyRandom::new();
    println!("randomized token {}", rng.rand_token(100));
    println!("success");
}

```

## 🎉 Have fun!

Whether you're building a game, a secure app, or just experimenting, my_random makes randomness easy and fun 😁

## More Resources

- 📖 [![Documentation](https://docs.rs/my_random/badge.svg)](https://docs.rs/my_random)
- 🧪 [See usage examples](https://github.com/JadeEmperor02/my_random/tree/master/examples)
- 🛠️ [Crate on crates.io](https://crates.io/crates/my_random)

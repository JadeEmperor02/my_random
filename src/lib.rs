use core::panic;
use std::time::{ SystemTime, UNIX_EPOCH };
// use std::thread;
use std::process;
pub struct MyRandom {
    state0: u64,
    state1: u64,
}

impl MyRandom {
    pub fn new() -> Self {
        let seed: u64 = generate_seed();
        let state0: u64 = seed;
        let state1: u64 = seed.wrapping_add(12345678910111213);

        if state0 == 0 && state1 == 0 {
            return Self { state0: 1, state1: 0 };
        }
        Self { state0, state1 }
    }

    fn next_u64(&mut self) -> u64 {
        let mut s1: u64 = self.state0;
        let s0: u64 = self.state1;
        self.state0 = s0;

        s1 ^= s1 << 23; //a
        s1 ^= s1 >> 17; //b
        s1 ^= s0; // c
        s1 ^= s0 >> 26; //d
        self.state1 = s1;
        s1.wrapping_add(s0)
    }

    pub fn rand(&mut self) -> f64 {
        const MAX_U64_PLUS_ONE: f64 = (u64::MAX as f64) + 1.0;
        (self.next_u64() as f64) / MAX_U64_PLUS_ONE
    }
    pub fn rand_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            panic!("invalid range");
        }

        let range: u64 = max - min;
        min + (self.next_u64() % range)
    }
    pub fn rand_vector<T: Clone>(&mut self, val: &[T], count: usize) -> Vec<T> {
        let mut indices: Vec<usize> = (0..val.len()).collect();
        let mut new_vec: Vec<T> = Vec::with_capacity(count);
        for _ in 0..count.min(val.len()) {
            let random_index: usize = self.rand_range(0, indices.len() as u64) as usize;
            let choosen = indices.remove(random_index);
            new_vec.push(val[choosen].clone());
        }
        new_vec
    }
    pub fn shuffle_vector<T>(&mut self, val: &mut [T]) {
        for i in (1..val.len()).rev() {
            let j: usize = self.rand_range(0, (i + 1) as u64) as usize;
            val.swap(i, j);
        }
    }
    pub fn rand_bool(&mut self) -> bool {
        self.rand_range(0, 2) == 1
    }
}

fn generate_seed() -> u64 {
    let time: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let pid: u64 = process::id() as u64;
    let addr: u64 = &time as *const u64 as u64;
    // let tid: u64 = thread::current().id().as_u64().unwrap_or(0); // unstable needs nightly
    time ^ pid ^ addr
}

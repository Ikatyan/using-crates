extern crate rand;

use rand::Rng;
use rand::FromEntropy;
use rand::rngs::{StdRng, SmallRng, EntropyRng, JitterRng, OsRng, ThreadRng};

fn use_stdrng() {
    let mut std_rng = StdRng::from_entropy();
    let x = (0..10).map(|_| std_rng.gen_range(1, 10)).collect::<Vec<i32>>();
    println!("{:?}", x);
}

fn use_smallrng() {
    let mut rng = SmallRng::from_entropy();
    let x = (0..10).map(|_| rng.gen_range(0, 10)).collect::<Vec<i32>>();
    println!("{:?}", x);
}

fn use_entropyrng() {
    let mut rng = EntropyRng::new();
    let x = (0..10).map(|_| rng.gen_range(0, 10)).collect::<Vec<i32>>();
    println!("{:?}", x);
}

fn use_jitterrng() {
    let mut rng = JitterRng::new().unwrap();
    let x = (0..10).map(|_| rng.gen_range(0, 10)).collect::<Vec<i32>>();
    println!("{:?}", x);
}

fn use_osrng() {
    let mut rng = OsRng::new().unwrap();
    let x = (0..10).map(|_| rng.gen_range(0, 10)).collect::<Vec<i32>>();
    println!("{:?}", x);
}

fn use_threadrng() {
    let mut rng = rand::thread_rng();
    let x = (0..10).map(|_| rng.gen_range(0, 10)).collect::<Vec<i32>>();
    println!("{:?}", x);
}

fn main() {
    use_stdrng();
    use_smallrng();
    use_entropyrng();
    use_jitterrng();
    use_osrng();
    use_threadrng();
    use_threadrng();
}
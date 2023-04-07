use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

mod generators {
    use fastrand;
    use nanorand;
    use rand;

    #[no_mangle]
    pub fn rand_(n: usize) {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        for _ in 0..n {
            rand::Rng::gen::<f64>(&mut rng);
        }
    }

    #[no_mangle]
    pub fn fastrand_(n: usize) {
        let rng: fastrand::Rng = fastrand::Rng::new();
        for _ in 0..n {
            rng.f64();
        }
    }

    #[no_mangle]
    pub fn smallrng_(n: usize) {
        let mut rng: rand::rngs::SmallRng = rand::SeedableRng::from_entropy();
        for _ in 0..n {
            rand::Rng::gen::<f64>(&mut rng);
        }
    }

    #[no_mangle]
    pub fn nanorand_(n: usize) {
        let mut rng: nanorand::WyRand = nanorand::WyRand::new();
        for _ in 0..n {
            nanorand::Rng::generate::<f64>(&mut rng);
        }
    }
}

fn generator_timer<F: Fn(usize)>(func: F, n: usize) -> Duration {
    let start: Instant = Instant::now();
    func(n);
    start.elapsed()
}

fn main() {
    let n: usize = 10_000_000;
    let generators: [(&str, fn(usize) -> ()); 4] = [
        ("rand", generators::rand_),
        ("fastrand", generators::fastrand_),
        ("smallrng", generators::smallrng_),
        ("nanorand", generators::nanorand_),
    ];

    let mut threads: Vec<JoinHandle<(&str, usize, Duration)>> = generators
        .into_iter()
        .map(|(name, func)| thread::spawn(move || (name, n, generator_timer(func, n))))
        .collect::<Vec<JoinHandle<(&str, usize, Duration)>>>();

    while !threads.is_empty() {
        let mut i: usize = 0;
        while i < threads.len() {
            if !threads[i].is_finished() {
                i += 1;
                continue;
            }

            let (name, n, duration) = threads.remove(i).join().unwrap();
            println!("{} generated {} random values in {:?}", name, n, duration);
        }
    }
}

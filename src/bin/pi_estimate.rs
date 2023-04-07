use std::f64::consts::PI;
use std::thread::{self, JoinHandle};
use std::time::Instant;

use rand::{self, rngs::SmallRng, Rng, SeedableRng};

const THREADS: usize = 12;

fn buffons_needle(n: usize) -> f64 {
    let chunk_size: usize = n / THREADS;
    let hits: usize = (0..THREADS)
        .map(|_| {
            thread::spawn(move || {
                let mut rng: SmallRng = SmallRng::from_entropy();
                let mut local_hits: usize = 0;

                for _ in 0..chunk_size {
                    let angle: f64 = rng.gen::<f64>() * PI * 2.0f64;
                    let tip: f64 = rng.gen::<f64>() + angle.sin();

                    if tip < 0.0f64 || tip > 1.0f64 {
                        local_hits += 1;
                    }
                }
                local_hits
            })
        })
        .collect::<Vec<JoinHandle<usize>>>()
        .into_iter()
        .map(|thread: JoinHandle<usize>| thread.join().unwrap())
        .sum();

    (2.0f64 * n as f64) / hits as f64
}

fn main() {
    for i in 0..=13 {
        let n: usize = 10usize.pow(i);
        let start: Instant = Instant::now();
        println!(
            "{} with {:+e} nails in {:?}",
            buffons_needle(n),
            n,
            Instant::now() - start
        );
    }
}

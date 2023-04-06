use std::cmp::Ordering;

fn sqrt(x: f64) -> f64 {
    if x == 0.0f64 {
        return 0.0f64;
    }

    let iteration_limit: usize = 64;
    let mut upper: f64 = 0.0005f64 * x + 500.0f64;
    let mut lower: f64 = 0.0f64;
    let mut mid: f64 = (upper + lower) / 2.0f64;

    for _ in 0..iteration_limit {
        match x.total_cmp(&mid.powi(2)) {
            Ordering::Equal => return mid,
            Ordering::Less => upper = mid,
            Ordering::Greater => lower = mid,
        };
        mid = (upper + lower) / 2.0;
    }
    mid
}

fn main() {
    println!("{}", sqrt(9.0));
    println!("{}", sqrt(2.0));
    println!("{}", sqrt(0.5));
    println!("{}", sqrt(0.25));
    println!("{}", sqrt(100.0));
    println!("{}", sqrt(81.0));
    println!("{}", sqrt(49.0));
    println!("{}", sqrt(0.0));
}

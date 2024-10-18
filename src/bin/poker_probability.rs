mod cards;

use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::cards::deck::Deck;
use crate::cards::hand::{ CATEGORIES, Category, Hand };

const THREAD_COUNT: usize = 24;

fn register(n: usize) -> [usize; CATEGORIES] {
    let mut result: [usize; CATEGORIES] = [0; CATEGORIES];

    let mut deck: Deck = Deck::full();
    for _ in 0..n {
        deck.shuffle();
        let hand: Hand = Hand::from_deck(&deck);
        result[hand.category().ordinal()] += 1; 
    }

    result
}

fn parallel_register(n: usize) -> [usize; CATEGORIES] {
    let group_size: usize = n / THREAD_COUNT;
    let rest: usize = n % THREAD_COUNT;
     
    let mut thread_results: Vec<[usize; CATEGORIES]> = (0..THREAD_COUNT).map(|_| thread::spawn(move || register(group_size)))
        .map(|thread: JoinHandle<[usize; CATEGORIES]>| thread.join().unwrap())
        .collect::<Vec<[usize; CATEGORIES]>>();
    thread_results.push(register(rest));
    
    thread_results.into_iter().fold([0; CATEGORIES], |mut acc, res| {
        acc.iter_mut().zip(res.iter()).for_each(|(a, b)| *a += b);
        acc
    })
}

fn register_timer<F: Fn(usize) -> [usize; CATEGORIES]>(func: F, n: usize) -> ([usize; CATEGORIES], Duration) {
    let start: Instant = Instant::now();
    let return_value: [usize; CATEGORIES] = func(n);
    (return_value, start.elapsed())
    
}

fn main() {
    let n: usize = 100_000_000;
    
    let (result, duration): ([usize; CATEGORIES], Duration) = register_timer(register, n);
    println!("func=reigster, n={} | Duration: {} ms", n, duration.as_millis());
    println!("----------------");
    result.iter()
        .map(|&count: &usize| (count as f64) / (n as f64))
        .zip(Category::all())
        .for_each(|(probability, category)| println!("{}: {:.6}%", category, probability * 100f64));

    println!("\n###############\n");
    
    let (parallel_result, parallel_duration): ([usize; CATEGORIES], Duration) = register_timer(parallel_register, n);
    println!("func=parallel_register, n={} [threads={}] | Duration: {} ms", n, THREAD_COUNT, parallel_duration.as_millis());
    println!("----------------");
    parallel_result.iter()
        .map(|&count: &usize| (count as f64) / (n as f64))
        .zip(Category::all())
        .for_each(|(probability, category)| println!("{}: {:.6}%", category, probability * 100f64));

}

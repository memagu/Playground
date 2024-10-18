use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Copy, Clone)]
pub struct XorShift {
    state: usize,
}

impl XorShift {
    pub fn from_seed(seed: usize) -> Self {
        Self { state: seed }
    }

    pub fn new() -> Self {
        let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as usize;

        Self::from_seed(seed)
    }

    pub fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub fn gen_range(&mut self, a: usize, b: usize) -> usize {
        a + self.next() % (b - a)
    }
}

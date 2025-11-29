#[derive(Debug)]
struct Accumulator {
    sum: i32
}

// implements methods for Accumulator
impl Accumulator {
    fn new(init : i32) -> Self {
        Self { sum : init }
    }

    fn get(&self) -> i32 { // borrows, does not move
        self.sum
    }

    fn add(&mut self, increment : i32) { // mutably borrows 
        self.sum += increment;
    }
}

fn main() {
    let mut acc = Accumulator::new(0);

    for i in 1..100 {
        acc.add(i);
    }

    println!("The acc is {:?}", acc.get());
}

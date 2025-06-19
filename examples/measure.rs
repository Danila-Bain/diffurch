use std::time::Instant;

fn main() {
    loop {
        let start = Instant::now();
        // Code block to measure
        let x = some_function();
        let duration = start.elapsed();
        println!("Execution time: {:?}, result: {x}", duration);
    }
}

fn some_function() -> f64 {
    let mut x = 1.5;
    for _ in 0..10000 {
        x = 1. - 1. / x;
    }
    x
}

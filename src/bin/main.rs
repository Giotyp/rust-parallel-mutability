use int_mut::threadlock::*;
use int_mut::lockfree::*;

fn main() {
    let n = 10;
    let threads = 5;

    println!("Running CRwLock implementation:\n");
    threadlock::run(n, threads);

    println!("\nRunning lock-free implementation:\n");
    lockfree::run(n, threads);
    
}

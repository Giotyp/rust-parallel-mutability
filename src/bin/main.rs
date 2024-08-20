use int_mut::threadlock::*;
use int_mut::lockfree::*;
use int_mut::multibuffer::*;

fn main() {
    let n = 10;
    let threads = 5;

    println!("1) Running CRwLock implementation:\n");
    threadlock::run(n, threads);

    println!("\n\n2) Running lock-free implementation:\n");
    lockfree::run(n, threads);

    println!("\n\n3) Running multi-buffer application:\n");
    multibuffer::run(n);
    
}

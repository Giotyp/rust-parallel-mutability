use std::sync::Arc;
use int_mut::structure::{AlignedVec, CRwLock};
use rayon::prelude::*;

fn print_vec(vec: &AlignedVec<CRwLock<i32>>) {
    let nums: Vec<i32> = (0..vec.get().len())
    .map(|i| *vec.get()[i].read())
    .collect();

    println!("{:?}", nums);
}

fn main() {
    let n = 10;
    let threads = 5;
    let mut algn_buffer: Arc<AlignedVec<CRwLock<i32>>> = Arc::new(AlignedVec::new(n, 64));

    // Initialize the vector
    let mut_vec = Arc::get_mut(&mut algn_buffer).unwrap();
    let mut init = 0;
    for i in 0..threads {
        let buf = mut_vec.get_mut();
        let start = i*n/threads;
        for j in start..(start + n/threads) {
            let mut num = buf[j].write();
            *num = init;
        }
        init += 1;
    }

    // Print the numbers in vector_count
    println!("Vector after initialization:");
    print_vec(&algn_buffer);

    algn_buffer.get().par_chunks(n/threads).for_each(|chunk| {
        for num in chunk {
            let mut num = num.write();
            *num += 1;
        }
    });

    // Print the numbers in vector_count
    println!("Vector after modification:");
    print_vec(&algn_buffer);
}

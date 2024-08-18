pub mod lockfree
{
    use crate::structure::AlignedVec;
    use rayon::prelude::*;

    pub fn run(n: usize, threads: usize) {

        // Initialize the vector 
        let mut algn_buffer = AlignedVec::new(n, 64);
        let chunk_size = n / threads;
        let mut init = 0;
        for i in 0..threads {
            let buf = algn_buffer.as_mut_slice();
            let start = i * chunk_size;
            for j in start..(start + chunk_size) {
                buf[j] = init;
            }
            init += 1;
        }

        // Print the initialized vector
        println!("Vector after initialization:");
        print_vec(&algn_buffer);

        let algn_slice = &mut algn_buffer.as_mut_slice()[..];
        algn_slice.par_chunks_mut(chunk_size).for_each(|chunk| {
            for num in chunk.iter_mut() {
                *num += 1;
            }
        });

        // Print the modified vector
        println!("Vector after modification:");
        print_vec(&algn_buffer);

    }

    fn print_vec(vec: &AlignedVec<i32>) {
        let nums: Vec<i32> = (0..vec.get().len())
        .map(|i| vec.get()[i])
        .collect();
    
        println!("{:?}", nums);
    }

}
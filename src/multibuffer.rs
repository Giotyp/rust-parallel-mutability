pub mod multibuffer {
    use crate::structure::AlignedVec;
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicPtr, Ordering};

    fn compute(buf_a: &[i32], buf_b: &[i32], buf_c: &mut [i32], buf_d: &mut [i32], elem: usize) {
        buf_c[elem] = buf_a[elem] + buf_b[elem];
        buf_d[elem] = buf_c[elem] * buf_b[elem];
    }

    unsafe fn compute_ptr(
        buf_a: &[i32],
        buf_b: &[i32],
        buf_c: *mut i32,
        buf_d: *mut i32,
        elem: usize,
    ) {
        unsafe {
            *buf_c.add(elem) = buf_a[elem] + buf_b[elem];
            *buf_d.add(elem) = *buf_c.add(elem) * buf_b[elem];
        }
    }

    pub fn run(n: usize) {
        let mut buf_c: AlignedVec<i32> = AlignedVec::new(n, 64);
        let mut buf_d: AlignedVec<i32> = AlignedVec::new(n, 64);
        let mut buf_a: AlignedVec<i32> = AlignedVec::new(n, 64);
        let mut buf_b: AlignedVec<i32> = AlignedVec::new(n, 64);

        for i in 0..n {
            buf_a.get_mut()[i] = 1 as i32;
            buf_b.get_mut()[i] = 2 as i32;
        }

        // Print operation
        println!("\nPerforming operations on vectors:");
        println!("C[i] = A[i] + B[i]");
        println!("D[i] = C[i] * B[i]\n");

        // Print the initialized vectors
        println!("Vectors after initialization:");
        print!("A = ");
        print_vec(&buf_a);
        print!("B = ");
        print_vec(&buf_b);


        // Parallel Chunks Implementation

        let buf_a_slice = &buf_a.as_slice()[..];
        let buf_b_slice = &buf_b.as_slice()[..];
        let buf_c_slice = &mut buf_c.as_mut_slice()[..];
        let buf_d_slice = &mut buf_d.as_mut_slice()[..];

        // Parallel addition across elements
        buf_c_slice
            .par_chunks_mut(2)
            .zip(buf_d_slice.par_chunks_mut(2))
            .for_each(|(c_slice, d_slice)| {
                for i in 0..c_slice.len() {
                    compute(buf_a_slice, buf_b_slice, c_slice, d_slice, i);
                }
            });

        println!("\nVectors after par_chunks_mut:");
        print_vec(&buf_c);
        print_vec(&buf_d);


        // Atomic Pointer Implementation

        let buf_c_ptr = buf_c.get_mut().as_mut_ptr();
        let buf_d_ptr = buf_d.get_mut().as_mut_ptr();

        let mut v = vec![];
        for i in 0..n {
            v.push((i, AtomicPtr::new(buf_c_ptr), AtomicPtr::new(buf_d_ptr)));
        }

        v.par_iter()
            .for_each(|(i, c_atomic_ptr, d_atomic_ptr)| unsafe {
                compute_ptr(
                    buf_a_slice,
                    buf_b_slice,
                    c_atomic_ptr.load(Ordering::SeqCst),
                    d_atomic_ptr.load(Ordering::SeqCst),
                    *i,
                );
            });

        println!("\nVectors after Atomic Ptr:");
        print_vec(&buf_c);
        print_vec(&buf_d);
    }

    fn print_vec(vec: &AlignedVec<i32>) {
        let nums: Vec<i32> = (0..vec.get().len()).map(|i| vec.get()[i]).collect();

        println!("{:?}", nums);
    }
}

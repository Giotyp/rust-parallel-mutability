pub mod multibuffer
{
    use std::sync::Arc;

    use crate::structure::{AlignedVec, CRwLock};
    use rayon::prelude::*;

    fn compute(buf_a: &[i32], buf_b: &[i32], buf_c: &mut [i32], buf_d: &mut [i32], elem: usize) {
        buf_c[elem] = buf_a[elem] + buf_b[elem];
        buf_d[elem] = buf_c[elem] * buf_b[elem];
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
        print!("A = "); print_vec(&buf_a);
        print!("B = "); print_vec(&buf_b);


        let buf_a_slice = &buf_a.as_slice()[..];
        let buf_b_slice = &buf_b.as_slice()[..];
        let buf_c_slice = &mut buf_c.as_mut_slice()[..];
        let buf_d_slice = &mut buf_d.as_mut_slice()[..];

        // Parallel addition across elements
        buf_c_slice.par_chunks_mut(2).zip(buf_d_slice.par_chunks_mut(2)).for_each(|(c_slice, d_slice)| {
            for i in 0..c_slice.len() {
                compute(buf_a_slice, buf_b_slice, c_slice, d_slice, i);
            }
        });


        // Print the resulting vector after addition
        println!("\nVectors after par_chunks_mut:");
        print_vec(&buf_c);
        print_vec(&buf_d);


        let arc_c: Arc<CRwLock<AlignedVec<i32>>> = Arc::new(CRwLock::new(buf_c));
        let arc_d: Arc<CRwLock<AlignedVec<i32>>> = Arc::new(CRwLock::new(buf_d));
        let arc_a = Arc::new(buf_a);
        let arc_b = Arc::new(buf_b);

        let ids = (0..n).collect::<Vec<usize>>();

        ids.par_iter().for_each(|i| {
            let mut c_slice = arc_c.write();
            let mut d_slice = arc_d.write();
            compute(&arc_a.as_slice(), &arc_b.as_slice(), &mut *c_slice.get_mut(), &mut *d_slice.get_mut(), *i);
        });

        // Print the resulting vector after multiplication
        println!("\nVectors after Arc mutation:");
        print_vec(&arc_c.read());
        print_vec(&arc_d.read());
 

    }

    fn print_vec(vec: &AlignedVec<i32>) {
        let nums: Vec<i32> = (0..vec.get().len())
        .map(|i| vec.get()[i])
        .collect();
    
        println!("{:?}", nums);
    }

}
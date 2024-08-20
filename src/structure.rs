use libc;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait GenType:  Clone + Default{}
impl<T: Clone + Default> GenType for T{}

#[derive(Clone, Default)]
pub struct AlignedVec<T: GenType> {
    buffer: Vec<T>,
}

impl<T: GenType> AlignedVec<T> {
    pub fn new(size: usize, alignment: usize) -> Self {
        let buffer = Self::create_aligned_vec(size, alignment);
        AlignedVec { buffer }
    }

    pub fn get(&self) -> &Vec<T> {
        &self.buffer
    }

    pub fn get_mut(&mut self) -> &mut Vec<T> {
        &mut self.buffer
    }

    pub fn get_mut_slice(&mut self, index: usize) -> &mut T {
        &mut self.buffer[index]
    }

    pub fn get_slice(&mut self, index: usize) -> &T {
        &self.buffer[index]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.buffer
    }

    pub fn as_slice(&self) -> &[T] {
        &self.buffer
    }


    fn create_aligned_vec(size: usize, alignment: usize) -> Vec<T> {
        let mut ptr: *mut libc::c_void = std::ptr::null_mut();
        let align_res = unsafe { libc::posix_memalign(&mut ptr, alignment, size * std::mem::size_of::<T>()) };
        if align_res != 0 {
            panic!("Failed to allocate aligned memory");
        }
        let ptr = ptr as *mut T;
        let mut vec = Vec::with_capacity(size);
        unsafe {
            for _ in 0..size {
                vec.push(std::mem::transmute::<_, T>(ptr.read()));
            }
        }
        vec
    }
}

impl<T: GenType> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        let buffer = std::mem::replace(&mut self.buffer, Vec::new());
        let _ = std::mem::ManuallyDrop::new(buffer);
    }
}


#[derive(Debug)]
pub struct CRwLock<T>(RwLock<T>);

impl<T: GenType> CRwLock<T> {
    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.0.write().unwrap()
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.0.read().unwrap()
    }

    pub fn new(data: T) -> Self {
        CRwLock(RwLock::new(data))
    }
}

impl<T: GenType> Clone for CRwLock<T> {
    fn clone(&self) -> Self {
        let data = self.read().clone();
        CRwLock(RwLock::new(data))
    }
}

impl<T: GenType> Default for CRwLock<T> {
    fn default() -> Self {
        CRwLock(RwLock::new(T::default()))
    }
}
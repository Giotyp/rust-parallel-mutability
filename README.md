# Parallel Buffer Modification with Rust

## Problem Description

Modifying a buffer in Rust using multiple threads could be a complex process.
In the case of a simple counting problem, where all threads need to modify the same value, a simple combination of **std::sync::{Arc, Mutex}** can be used. 
However, when multiple threads need to modify separate elements, this process adds excess latency, as threads need to wait for mutex access.
If we used **C**, we could easily utilize pointers to various buffer indexes, so that each thread could modify a different buffer section.
Safe Rust prevents this solution, as it dictates that multiple immutable references to a value can be created but only a single mutable one.
So, how do we design a safe and simple solution to modify different sections of a buffer in parallel?

## Proposed Solutions

1) #### Lock Implementation 
    Following a series of posts named [Interior mutability in Rust](https://ricardomartins.cc/2016/06/08/interior-mutability), by Ricardo Martins, we can utilize **std::sync::RwLock** for our purpose. 
    By creating a buffer of type *Arc\<Vec\<RwLock\<T>>>* we can modify all elements of the vector concurrently, without having to explicitly call unsafe operations.

2) #### Lock-Free Implementation
    In case we want to complemetly avoid locks, to maximize performance, we can utilize Rust's **slicing**.  By obtaining a mutable slice to our buffer, and using Rayon's **par_chunks_mut**, we can get lock-free access to the different non-overlapping buffer regions. 

3) #### Multiple Mutable Buffers
    When he have multiple buffer, we can again utilize Rayon's **par_chunks_mut**, combined with **zip** function, to iterate through mutable slices of the buffers. Another approach is to use an *AtomicPtr* structure and obtain mutable pointers from each thread.


## Code Example

A simple example is implemented in  files [threadlock.rs](src/threadlock.rs) & [lockfree.rs](src/lockfree.rs), for cases 1 and 2 respectively, with a vector containing 32-bit integers. Custrom structs for the vector and the RwLock are used to allow for more flexibility.
The buffer is initialized with the values [0, 0, 1, 1, 2, 2, 3, 3, 4, 4] and each thread is supposed to modify two adjacent elements. 
The parallization is achieved using the [Rayon](https://crates.io/crates/rayon) crate.
In the end, the buffer should be in the state [1, 1, 2, 2, 3, 3, 4, 4, 5, 5].

In [multibuffer.rs](src/multibuffer.rs), an addition and a multiplication operation is performed in two buffers C, D, by using elements from buffers A, B. The operation is performed either with **par_chunks_mut** and **zip**, or with an **Atomic Pointer**. The *AtomicPtr* structure provides flexibility, when we want to mutate random elements of the given buffers from each thread. 

## Custom Structures

### GenType
A wrapper trait implementing needed traits. It is used solely for code simplicity.

### AlignedVec
A vector with given memory alignment.
It utilizes low-level **libc** operations to store an aligned buffer in memory.
This structure is useful when operations require aligned buffers (like SIMD operations in Intel processors). 

### CRwLock
Wrapper for RwLock that implements extra traits (**Clone**), which are not supported by RwLock.
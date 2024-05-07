# Parallel Buffer Modification with Rust

## Problem Description

Modifying a buffer in Rust using multiple threads could be a complex process.
In the case of a simple counting problem, where all threads need to modify the same value, a simple combination of **std::sync::{Arc, Mutex}** can be used. 
However, when multiple threads need to modify separate elements, this process adds excess latency, as threads need to wait for mutex access.
If we used **C**, we could easily utilize pointers to various buffer indexes, so that each thread could modify a different buffer section.
Safe Rust prevents this solution, as it dictates that multiple immutable references to a value can be created but only a single mutable one.
So, how do we design a safe and simple solution to modify different sections of a buffer in parallel?

## Proposed Solution

Following a series of posts named [Interior mutability in Rust](https://ricardomartins.cc/2016/06/08/interior-mutability), by Ricardo Martins, we can utilize **std::sync::RwLock** for our purpose. 
By creating a buffer of type *Arc\<Vec\<RwLock\<T>>>* we can modify all elements of the vector concurrently, without having to explicitly call unsafe operations.


## Code Example

A simple example is implemented in [main.rs](src/bin/main.rs) with a vector containing 32-bit integers. Custrom structs for the vector and the RwLock are used to allow for more flexibility.
The buffer is initialized with the values [0, 0, 1, 1, 2, 2, 3, 3, 4, 4] and each thread is supposed to modify two adjacent elements. 
The parallization is achieved using the [Rayon](https://crates.io/crates/rayon) crate.
Using the **par_chunks** directive we instruct each thread to increment each value on its acquired buffer range. 
In the end, the buffer should be in the state [1, 1, 2, 2, 3, 3, 4, 4, 5, 5].

## Custom Structures

### GenType
A wrapper trait implementing needed traits. It is used solely for code simplicity.

### AlignedVec
A vector with given memory alignment.
It utilizes low-level **libc** operations to store an aligned buffer in memory.
This structure is useful when operations require aligned buffers (like SIMD operations in Intel processors). 

### CRwLock
Wrapper for RwLock that implements extra traits (**Clone**), which are not supported by RwLock.
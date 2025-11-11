# beaumont
Algorithmic trading for quantitative financial applications in [Rust](https://rust-lang.org/) language.


> [!WARNING]  
> This project is in an early stage of development, and <strong>not</strong> production ready yet.
> Use with caution!



This project has been built and published as a reusable crate to [crates.io](https://crates.io/crates/beaumont), with its documentation made available at [docs.rs](https://docs.rs/beaumont), and it provides basic implementations for numbers, linear algebra, and financial algorithms. 

## numbers
A basic implementation of decimal numbers in fixed-point arithmetic.

## linear algebra
A basic implementation of linear algebra common operations.

### vectors
```rust
use beaumont::*;
    
let v1 = vector![1, 2, 3];                  

// Comparing for equality                     
assert_eq!(v1, vector![1, 2, 3]);             
                                              
// Negation                                   
let negated = v1.neg();                       
assert_eq!(negated, vector![-1, -2, -3]);     
                                              
// Scalar multiplication                      
let v2 = v1.scale_by(2);                      
assert_eq!(v2, vector![2, 4, 6]);             
                                              
// Addition                                   
let v3 = Vector::from([1, 2, 3]);             
let v4 = v1.add(&v2);                         
assert_eq!(v4, vector![3, 6, 9]);             
                                              
// Dot product                                
let s = v1.dot(&v2);                          
assert_eq!(s, 28);                            
```

### matrices
...

## algorithms
...
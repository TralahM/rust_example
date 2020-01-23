#![macro_use]

#[macro export]
macro_rules! welcome{
 ()=>(
    println!("Welcome to Rust Macros");
 )
}


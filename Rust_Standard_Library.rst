Rust's Standard Library
=============================

- The Prelude Module.

  List of thingd that Rust automatically imports into every program

1. Rust provides a powerful macro system that allows metaprogramming

2. Rust Macros are non conflicting

.. code:: rust


  #![macro_use]

  #[macro export]
  macro_rules! welcome{
     ()=>(
        println!("Welcome to Rust Macros");
     )
  }

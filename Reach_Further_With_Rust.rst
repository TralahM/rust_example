Rust's Standard Library
=============================

Reaching Further with Rust
---------------------------
High-level code,low-level performance

Eliminate tradeoff between control and flexibility.

Buffer overflows,Double free,Dangling POinters, Data races

Static type system = Eat your spinach!

.. code:: rust

   extern "C" fn fast_blank(buf: Buf) -> bool {
      buf.as_slice().chars().all(|c| c.is_whitespace())
   }

   // High-level, zero-cost abstractions
   fn is_whitespace(text: &str) -> bool {
      text.chars().all(|c| c.is_whitespace())
   }

   fn load_images(paths: &[PathBuf]) -> Vec<Image> {
      paths.par_iter()
         .map(|path| Image::load(path))
         .collect()
   }

Design of Rust
================
Mozilla developers faced issues in multithreading like

- Sharing memory

- Mutation

- No ordering

Or more collectively **Data Race**

Sharing and Mutation use is mutually exclusive enforced by the concept of **Ownership and Borrowing**

.. code:: rust

   // The function Main owns the apple
   fn main(){
      let apple=Apple::new();
      //Give Ownership of the apple
      eat(apple);
      eat(apple);//fails error as apple has already been moved
   }

   // Takes ownership of apple
   fn eat(apple:Apple){
      println!("Eaten {:?}",apple);
   }

   fn main(){
      let apple=Apple::new();
      let mut bag=Vec::new();
      bag.push(apple);
      bag.push(Apple::new());
      deliver(bag);
      // Share or Loan out bag
      let weight=weigh(&bag);
   }

   fn deliver(bag: Vec<Apple>){
      //.....
      //when final owner returns value is freed
   }
   fn weigh(bag: Vec<Apple>) -> u32{
      //.....
   }

Borrowed values are immutable
After last use of borrowed value is used mutability is restored

Mutable References
***********************
.. code:: rust

   let mut bag=Vec::new();
   bag.push(...); // bag mutable;
   let r= &mut bag; //bag mutably borrowed
   bag.len();// Cannot access bag while borrowed
   r.push(..); //but can mutate through r
   bag.push(...); //at last bagg is accessible again

Parallellism
**************
Building parallel abstractions is easy

Misusing those abstractions is also easy

.. code:: rust

   fn foo(...){
      let m=HashMap::new();
      m.insert("Hello","World");
      channel.send(m);
      m.insert("Hello","Data Race!"); //error.use of moved value
   }
   //Method
   impl<T> Channel<T>{
      fn send(&mut self,data: T){ //Take Ownership of data
         ...
      }
   }

Concurrency Paradigms
**********************

=========         ================  ================
Paradigm          Ownership?        Borrowing?
=========         ===============   ================
Message Parsing   YES               NO
Fork Join         NO                YES


Rust in Production
===================
Rust Community
==============
Modules in the std library
----------------------------
- The Prelude Module.

  List of things that Rust automatically imports into every program

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

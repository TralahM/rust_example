Introduction to Rust Programming
====================================
- Created by Mozilla Devs
- Good for chat clients,games,servers
- System Programming Candidate
- Designed for easy simple multithreading to address common issues faced by programmers
- C/C++ family  code blocks delimited with {}.
- Strict types language
- LLVM Based

Getting started with rust
------------------------------
- Data types are immutable by default
- WASM  webassembly

- **cargo**  package manager for Rust
.. code:: bash

   # The Rust Compiler
   rustc --version
   # The Package Manager|Build Tool
   cargo --version

Functions
----------
.. code:: rust

   fn main(){
      println!("Text");
   }

Projects
-----------
.. code:: bash

   cargo new project_name
   cd project_name
   ls
   #Cargo.toml, src,
   cargo run build
   # OR
   cargo run

Variables,Types and Such
-------------------------
.. code:: rust

   let x=32;//immutable
   let mut y=32;//mutable
   println!("x= {} \n y={}",x,y);//macros
   //x=43; causes error as x is immutable
   y=231;//is ok as y is mutable

* Integers 1
* Floats 2.3
* Booleans true || false
* Strings[Characters]

Basic Math
-------------
.. code:: rust

   let dyn_math= 8*8-2+221;
   pritnln!("Dynamic math {}",dyn_math);
   let my_array=[1,2,3,4,5,6,7];
   let my_tuple=(,42,34.3,"tralah");
   let (dyn_x,dyn_y,dyn_z)=my_tuple;//tuple unpacking as python
   // Array Indexing
   println!("{}",my_array[3]);

Crates
---------
Third Party Libraries
for
- Games
- Math
- Networks
- Graphics [ OpenGL ]

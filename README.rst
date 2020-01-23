Introduction to Rust Programming
====================================
- Created by Mozilla Devs
- Good for chat clients,games,servers
- System Programming Candidate
- Designed for easy simple multithreading to address common issues faced by programmers
- C/C++ family  code blocks delimited with {}.
- Strict types language
- LLVM Based

- Comparable to **CYCLONE** a programming language without a garbage collector

- High Level Abstraction

- Build System

- Data Race Free (!?)

A Case for Oxidation (Rust)
----------------------------

Memory Safety
*******************
Dereferences always succeed, always points to values of a correct type
if r:&Foo then * r is always equal to Foo
- No dangling pointers

- No access after memory free

- Forced Initialization+ Restricted aliasing+ Ownership = memory safety

All at *compile-time*

Aliasing Is hard!  avoid **segmentation faults** like in C/C++
So  we restrict aliasing so only 1 mutable alias or N immutable aliases

Borrowing
^^^^^^^^^^^

.. code:: rust

   let x=Person("Tralah");
   let y=&x;
   let z=&x;
   //let z=x;  would fail as x is already borrowed
   /* Ownership
    *let x=32;
    *let y=x;
    *let z=x;fails as x already belongs to y
   */

Concurrency
^^^^^^^^^^^^^
Not supported natively by the language because of the safe mem feature of Rust
Only Provided by Third Party Libraries which do this by using a concept of unsafe nemory
or unsafe rust.

- Minimal Runtime

- StrongType System

- Performance


Why Should we Care?
---------------------
We want secure systems
We want the right tools
C/C++ is old and ill designed, have to write Makefiles and toolchains
Prevents us from doing the wrong things
Good for writing System Software
Guaranteed Security

Guarantees
-------------
Memory safety without garbage collection
No data races

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

Functions and Macros
--------------------
.. code:: rust

   // C like Comments
   /*Multiple
      * Line
      Comments*/
   fn main(){
      println!("Text");
      add(21,34);
   }
   fn add(x: &int,y: &int){
      println!("{} + {} = {}",x,y,x+y);
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

Basic Types, Loops
--------------------------
.. code:: rust

   let dyn_math= 8*8-2+221;

   const NAME:str="Tralah M Brian";
   const ID: i32 =001;
   println!("{}",NAME);
   println!("{}",ID);

   pritnln!("Dynamic math {}",dyn_math);

   let my_array=[1,2,3,4,5,6,7];

   let my_tuple=(,42,34.3,"tralah");

   let (dyn_x,dyn_y,dyn_z)=my_tuple;//tuple unpacking as python

   // Array Indexing
   println!("{}",my_array[3]);

   // Array Looping
   for i in my_array.iter(){
      println!("{}",i);
   }

Crates
---------
Third Party Libraries for

- Games

- Math

- Networks

- Graphics [ OpenGL ]

Rocket
*********
Web Framework written is rust makes it secure by avoiding

- XSS,

- Directory Travesals,

  .. code:: rust

     #[get("/<path>")]
     fn retrieve(user: User,pid: PastebinId){
      File::f=12;
     }

- Remote Code Exec

- Sql Injection

- Authentication

- Authorization

- CORS

- Mosconfiguration

- Input Validation

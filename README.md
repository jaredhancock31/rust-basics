
### rustup
Rust toolchain installer.
`rustup update` Updates to latest rust version

### Cargo
Rust's build system and pkg manager. It can download and build code, libs, and dependencies
```
cargo new hello_cargo
cd hello_cargo
```
Creates new dir and creates project files for a new project named "hello_cargo"
 
 ##### Cargo.toml
 ```
[package]
name = "rust"
version = "0.1.0"
authors = ["Jared Hancock <email@place.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```
Houses the configuration information Cargo needs to compile your program. 

Dependencies are where you list projects dependencies, also known as *crates*. This is similar to gradle.

Cargo expects source files to live inside the *src* directory

##### Building & Running
`cargo build`
Compiles and creates an executable file in *target/debug/hello_cargo*

Also creates a new file at the top level: *Cargo.lock*. This file keeps track of the exact versions of the dependencies in your 
project. Shouldn't ever change this file manually.

`cargo run`
Only re-compiles if there are no changes since the last time.

`cargo check` Compiles without producing an executable


##### Building with profiles
`cargo build --release` Builds with optimizations and creates the executable in *target/release* instead of *target/debug*.
These optimizations make your actual Rust code run faster, but in turn lengthens your compile time. So there are two different 
profiles - one for development, and one for release. Any benchmarking should be done with the release profile.

##### Modules and Scope Control
*Modules* let us organize code within a crate into groups for readability and easy reuse. They control the privacy of items. 


## Ownership
Removes the need for GC. Memory is managed through a system of ownership and a set of rules that the compiler checks at compile
time.

##### Rules
* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

##### Stack & Heap
Stack stores values in the LIFO order, like a stack of plates. All data stored on the stack must have a known, fixed size. Data
with an unknown size at compile time or a size that might changed must be stored on the heap instead. The heap is less organized:
when you put data on the heap, you request a certain amount of space. The OS finds an empty spot in the heap big enough, marks it 
as in-use, and returns a pointer. Pointers are a known, fixed size, so they are stored on the stack. But when you want the actual
data, you must follow the pointer. 

### Traits
Similar to interfaces in Java

## Cheatsheet reference

### Variables
Rust is statically typed, but can usually infer what type we want to use based on the value and how it is used. In cases when many
types are possible, as seen below, we must add a [type annotation](https://doc.rust-lang.org/book/ch03-02-data-types.html) 
(like metadata for the variable).

`let guess: u32 = "42".parse().expect("Not a number!");`


##### Mutability
Variables are *immutable* by default
```markdown
let foo = 5; // immutable
let mut bar = 5; // mutable
```

```
let mut guess = String::new();
```
The `::` syntax indicates that `new` is an associated function of the `String` type. This is the same as like a static method for 
other languages. 

```markdown
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```
The `&` syntax indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access 
one piece of data without needing to copy that data into memory multiple times. 

##### Looping
`loop` Keyword for creating an infinite loop. `break` exits the loop.



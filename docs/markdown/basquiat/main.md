# Basquiat - Main Title

```rust
extern crate colored; 
extern crate radiancy; 

use colored::*; 
use radiancy::blockchain::blockchain::{
    Blockchain, new_blockchain
};

fn main() {
    let mut blockchain: Blockchain = new_blockchain();        
    blockchain = blockchain.add_block(String::from("This is Major Tome to Ground Control."));
    blockchain = blockchain.add_block(String::from("I am stepping through the door."));
    for _block in blockchain.blocks {
        println!("\nPrev. hash: {}", _block.prev_block_hash.yellow());
		    println!("Data: {}", _block.data.magenta());
		    println!("Hash: {}", _block.hash.cyan());
		    println!("Time: {}\n", _block.timestamp.underline().blue());
    };
}
```

+ Crate
+ Function
+ Loop
+ Macros
+ Struct
+ Syntax
+ Types
+ Variable

## Crate

```rust
extern crate colored;
```
Just like `import colored` in other languages.

## Function
```rust
fn main() {}
```
Function in Rust, the word is reduced to `fn`. Associate with `func`, `function`... in other languages.

## Loop

```rust
for _block in blockchain.blocks { code };
```

`for var in expression { code }`, for loop in rust. Rust also gets `for var i in arr { code }` format for loop and while loop, etc.

## Marcos

```rust
 println!("\nPrev. hash: {}", _block.prev_block_hash.yellow());
```
`println!` is a inside macro. rust also has `vec!`, `assert`... common macros.

## Struct

```rust
let mut blockchian: Blockchain = new_blockchain();
```
This file doesn't contain a struct defination. It has the struct types usages. `Blockchain` is the name of the struct. I'll explain struct more when we meet the complex struct using in following chapters.

## Syntax

Most sentences we got has the end `;` but `fn main() {}` not. Rust only have two types of Syntax.
One is `sententce` and the other is `expression`. `;` means the end of a sentence.

## Types

```rust
fn main() {} // function
let mut blockchain: Blockchain = new_blockchain(); // Blockchain
```
Types are quite complex. But if we have experiences in other language, types are peace of cake.

## Variable

```rust
let mut blockchian: Blockchain = new_blockchain();
```

With the design of `Ownership`, `Variable` might be the most magic part in rust. 

`let var = value` is the basic method of defining a variable. Instead of `let`, we alse have `const`, `static` which have other traits you'v already know. If some part make you confused, I guess is the meaning of the word `mut`.

Rust, which is famous with the `safe-RAM` trait while it says that Rust is The biggest competitor of `C++`. Rust work on the variable `ownership` of internal storage. Every time we pass value, we moved the value and release the internal storage. Don't worry, Rust offer lots of methods to deel with the design as the same. For more information, please read the [docs](https://doc.rust-lang.org).

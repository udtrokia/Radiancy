# Basquiat - Block

```rust
struct Block {}
impl Block {
    fn set_hash()
}

fn new_block(data: String, prev_block_hash: String) -> Block {}

fn new_genesis_block() -> Block {}

fn fn ts() -> String{}

```

+ Crate
  + Method Example
+ Struct 
  + Content Ownership
+ Variable Address

## Crate
```
use mod::mod::mod
```
To understand the crate in rust, I suggest to read the doc. it's easy and we can write pretty right crate soon with few mistakes.

But anyway, don't forget paying attention to the mod `path`.

#### Method example

```rust
let mut hasher = Sha256::new();
// No worry about the word `mut`, I'll explain it in `Variable Address`.
```
Create a new hasher by using the method `new()` of Sha256, the expression is `::`.

## Struct

```rust
struct Block {
    timestamp: String,
    data: String,
    prev_block_hash: String,
    hash: String
}
```
Writing out the struct is not defficult, but the use of it is difficult.

+ How to Add a Function in exist `struct?`

Well, the answer is `impl`

```rust
impl Block {
    fn set_hash(self) -> Block {}
}
```

But what I want to explain more is: `struct` is not a basic types. The `content` or `field` of struct can't change after we have defined it. 

What we need to learn is how to quote the value of a exsit `struct`, even change it? How about Copy a struct? There are plenty of methods waiting us to find them, the following is one of them.

#### Struct Content Ownership

```rust
fn set_hash(self) -> Block {
    let mut hasher = Sha256::new();
    let header: String = String::new()
        + &self.timestamp + &self.data + &self.prev_block_hash;
    hasher.input_str(&header);
    
    return Block {
        hash: String::from(hasher.result_str().as_str()),
        ..self
    }
}
```

In the file `set_hash`, we return a new struct to the variable(Type is Block). And the value of the preview struct had moved into the new struct then released. This process is a nice implement accrod the principle of rust `ownership`.

```
[Block]
    [Block.set_hash() =>
        {Block} with new_hash
```

__the old Block Didn't changed but released.__

__the new Block inherited parts of the old with `...self`, but with a different hash.__

> You can try google, `Rust ownership`, `Rust move value`, `Rust value point` or directly the bug you meet.

#### Variable Address

The last and the important. Here is an example to create a variable and change, copy, or quote it.

+ Define a variable.

```rust
let mut hasher = Sha256::new();
```
Wait... why I added mut in the line? 

__Here is a question of `Mutable`, `Copy`, `Clone`, `&`: what the hell with these methods?__
> Recommend to read the docs, and I'll explain all of them in `Blockchain` field.

Acutally, mut `receive` the value of right side. And give us authority to change the value at the exact internal storage.

## Summary


+ __We need to be conscious of the ownership in Rust in high-level types.__

+ __Methods(Copy, Clone, mut, &) can be mixed up easily, take care of them.__

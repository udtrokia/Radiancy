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
就像其他语言中的 `import colored` 。

## Function
```rust
fn main() {}
```
Rust 中的函数, 被缩减为 `fn`。 可以想象其他语言中的 `func`, `function`... 

## Loop

```rust
for _block in blockchain.blocks { code };
```

`for var in expression { code }`, 这个是 Rust 中的循环. Rust 也有像 `for var i in arr { code }` 这样格式的循环，与其他语言并没有太大差别。

## Marcos

```rust
 println!("\nPrev. hash: {}", _block.prev_block_hash.yellow());
```
`println!` 是一个内置的宏. Rust 还有 `vec!`, `assert` 等常用的内置宏。

## Struct

```rust
let mut blockchian: Blockchain = new_blockchain();
```
这个文件中并没有 `struct` 的定义，所以我仅仅在这里简单提及一下。

## Syntax

大多数句子的结尾为 `;` 但是 `fn main() {}` 并不是. Rust 仅仅有两种句法.
一种是 `语句` 另一种则是 `表达式`. `;` 表示语句的结束。

## Types

```rust
fn main() {} // function
let mut blockchain: Blockchain = new_blockchain(); // Blockchain
```
`Types` 也是令人头疼的一部分. 但是如果你有其他语言的经验，这并不是什么难事。

## Variable

```rust
let mut blockchian: Blockchain = new_blockchain();
```

在 Rust 的所有权体系中, `Variable` 可能会是很神奇的一部分。

`let var = value` 是定义变量的基本方法。 除去 `let`, 像我们之前所掌握的，还有 `const`, `static` 。如果还有什么让我们疑惑的地方，大概就是 `mut` 这个单词了吧。

Rust 因为它的内存安全特性，被称为 `C++` 的最大竞争者。关于 Rust 的变量所有权问题: `每当我们看到一个 =` 号，都有一次变量传递。将右边的值传递给了左边的变量，变量的地址并没有改变，改变的仅仅的存储变量的名称的地址，也就是指针的地址。当然 `Rust` 也内置了很多方法以便我们更好的运用这一特性。如果想要获得更多的了解，还是推荐读一读[docs](https://doc.rust-lang.org)。

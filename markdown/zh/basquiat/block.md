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
Rust 中的 `crate` 用来引入外部文件的内容，像其他语言中的 `import`，如果需要系统的了解，推荐看一下官方文档。编写包与包引入时的路径，或许需要我们特别注意一下。

#### 关于引入包的使用方法

```rust
extern crate crypto;
use crypto::sha2::Sha256;
let mut hasher = Sha256::new();
// No worry about the word `mut`, I'll explain it in `Variable Address`.
```
这里我们使用了`Sha256` 的 `new` 方法来创建一个新的 `hasher`。

## Struct

```rust
struct Block {
    timestamp: String,
    data: String,
    prev_block_hash: String,
    hash: String
}
```
在 `rust` 中，定义一个 `struct` 与其他语言相比并没有什么特别之处。但是由于它的特性，`struct` 的使用会有一些麻烦。

+ 如何为已经定义好的 `struct` 扩展方法？

使用 `impl` 来为已存在的 strcut 扩展方法。

```rust
impl Block {
    fn set_hash(self) -> Block {}
}
```

`strcut` 并不是像 `str`, `数字`, `数组` 之类的基本类型，在它被定义之后，所有的 `field` 都被分配进了固定的内存，可以借用，但是无法改变。除非你复制出来一个新的 `struct`, 并在新的 `struct` 中对它们进行改变。

在下面的例子，我们复制出来一个 `struct`, 并且对它进行改变。

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

我们返回了一个 `Type` 为 `Block` 的全新 `struct`。之前的 `struct` 把值移动到了新的 `struct` 当中，并且释放了这些值在之前地址的内存占用。这样的一个过程实际上也是有关 Rust 所有权特性的一次实践。


```
[Block]
    [Block.set_hash() =>
        {Block} with new_hash
```

__值被移动后，旧的`Block`并没有改变，但是被释放了。__

__新的`Block`更新了`hash`熟悉，并且通过`...self` 继承了之前`Block`的其他属性。__

> You can google: `Rust ownership`, `Rust move value`, `Rust value point` or directly the bug you meet.

#### Variable Address

变量的地址是 Rust 中很重要的一个概念。

+ 定义一个变量。

```rust
let mut hasher = Sha256::new();
```
为什么我的代码中有一个 `mut`?

__关于 `Mutable`, `Copy`, `Clone`, `&`，这些东西也有很大的学问。__

> 想要详细的了解, 可以看一下文档，或者在网上搜索相关资料。我会随着教程的进展，将这些内容一一解释出来。

`mut` 的作用在于: 接收等号右边的值，并创建一个新的内存用来储存我们的变量，并不对原有变量进行修改。与引用 `&` 相比，引用仅仅是引用之前的地址存储的变量。而 mut 创建出来了一个新的。

`Copy` 与 `mut` 还有一些不同，`Copy` 同样复制出来了一个新的变量，并且引用了更深层次的地址。就是一个全新的变量，`mut` 创造出来的变量，属性中会有 `mut xxx` 的标识。


## Summary


+ __我们可能要特别注意一下 Rust 中高级类型的使用方法。__

+ __(Copy, Clone, mut, &) 的使用方法非常容易被混淆。__

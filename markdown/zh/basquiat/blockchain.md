# Basquiat - Blockchain

```rust
use blockchain::block::{
    Block, new_block, new_genesis_block
}; 

pub struct Blockchain { pub blocks: Vec<Block> } 

impl Blockchain {
    pub fn get_prev_hash(&self) -> String {
        let prev_block: &Block = &self.blocks[self.blocks.len() -1];
        let prev_hash: String = (&prev_block.hash).to_string();
        return prev_hash;
    }
    pub fn add_block(mut self, data: String) -> Blockchain {
        let _prev_hash: String = self.get_prev_hash();
        let _new_block: Block = new_block(data, String::from(_prev_hash));

        self.blocks.push(_new_block);
        return Blockchain {
            blocks: self.blocks
        };
    }
}

pub fn new_blockchain() -> Blockchain {
    let mut _new_blockchain = Blockchain {
        blocks: vec![new_genesis_block()]
    };
    return _new_blockchain;
}
```

+ Vector
+ Borrow && Self

## Vector 

```rust
pub struct Blockchain { pub blocks: Vec<Block> }
```

`Vector` 与 `array` 相似, 但它是一个动态的数组，我们可以扩展它或者缩减它，这些是 Rust 中的数组类型没有的方法。

下面我们来定义一个 `Vec`:

```rust
let x: Vec<T> = vec![..]
```
+ Vec<T> 代表一个包含 `<T>` 类型的数组。
+ `vec!` 是创建 `Vector` 的宏。

## Borrow && Self

#### Borrow

在不熟悉Rust的情况下，引用这个特性会给我们带来很多麻烦。在这篇文章中，我仅仅解释我们用到的这一个。

```
pub fn get_prev_hash(&self)
```

如果你还记得 `mut` 意味着什么，我想你大概已经知道我要说什么了。我们引用了之前变量存储地址，也就是借用了之前的变量 `&` 这个符号用来表示借用。

#### Self

```rust
impl Blockchain {
    pub fn get_prev_hash(&self) -> String { 
    
    }
}
```

`self` 是当前对象的类型表示，它会出现在 `impl` 或 `trait` 里面，当然更多的是在 `trait` 中出现。

希望大家可以试一试下面的这个例子，这样我们就可以理解，在 `struct` 当中，`self` 到底意味着什么:

```rust
struct SomeStruct { some_string:  };
impl SomeStruct {
    pub fn some_function (&self) {
        let bar: &SomeStruct = self;
        println!("{:?}", bar);
        // Correct
        
        let bar: SomeStruct = self;
        println!("{:?}", bar);
        // With ISSUE.
    }
}
fn main() {
    let some_var: SomeStruct = SomeStruct { some_string: "david bowie".to_string() }.
    some_var.some_function();
}
```

在这个例子中, `fn some_function` 中的 `self` 与 `&self` 输出了同样的值，但是他们表示的类型不一样，存储的地址也不一样。

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

`Vector` seems like the basic type `array`, what make them different is: We don't need to define the arrays` content specificly. We can change it, expand it, even more reduce it.

The way we define a `Vec` is:

```rust
let x: Vec<T> = vec![..]
```
+ Vec<T> means a series Vector of type `<T>` type.
+ `vec!` is a macro to create `Vector`

## Borrow && Self

#### Borrow

Borrow covers a wide range of issues. This article we only introduce one aspect. If want to read the complete, I recommend you to read my independent rust notes or read the docs.

```
pub fn get_prev_hash(&self)
```
If you remember what happened to `mut`. I think you've already knowed what does `Borrow` exactly mean. I use the value with the internal storage we have already setted. `&some_var` means `Borrow`.

#### Self

```rust
impl Blockchain {
    pub fn get_prev_hash(&self) -> String { 
    
    }
}
```

`self` is the type of the current object. It may appear either in a trait or an impl, but appears most often in trait.

I hope you try this template with me, and you'll know what happened to `self`. Look at it:

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

In the template, `self` inner `fn some_function` means `&self` while the function defining.

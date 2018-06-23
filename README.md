# Introduce



## SUMMARY



Radiancy is a toy blockchain what I regard as a blockchain playground. Five built-in commands can help us taste the taste of  blockchain honey. It's actually about... proof of work, database, crypto transaction and wallet —— A relatively completely transaction system but lack of coinbase handlings, consensus and network.

In order to ensure every command runs correctlly, Radiancy will generate a database and a wallet in current path whatever you have inputted 😂

Anyway, Radiancy is simple and easy to use... And I wish all of Radiancy users get a pretty good experience with Radiancy.

That is... Welcome to the blockchain world,  and Long Live Rock n Roll.



## INSTALLATION

+ Cargo

```
cargo install radiancy 
```

+ Shell

```
~ curl https://udtrokia.github.io/homebrew-udtrokia/scripts/radiancy.sh | sh 
~ radiancy help
```



+ Build from source code

```
~ git clone git@github.com:udtrokia/Radiancy.git
~ cd /path/to/radiancy/ && cargo build
```



## USAGE



```
 ℷ  radiancy

<-- Hello Yellow Brick Road -->

Usage: radiancy COMMAND;

COMMANDS:
    create_blockchain         Generate a blockchain;
    create_wallet             Generate a wallet;
    get_balance               Get address balance;
    print_chain               Print blocks in Radiancy;
    print_address             Print blocks in Radiancy;
    send                      Send coin between addresses;

<-- GoodBye Yellow Brick Road -->
```





## COMMAND



### create_blockchain

> Generate a blockchain with db and default wallet.



__Example__

```
radiancy create_blockchain
```

Actually, in the current version, this command is useless: `create both db and wallet in the current path.`



### create_wallet

> Generate a wallet file in the current path.



__Example__

```
radiancy create_walelt
```

And then, you will find a file named `account.rdc` in the current path.



### get_balance 

> Get the balance of current account or the following.



__Example__

+ current account

```
radiancy get_balance
```

+ address as a parameter

```
radiancy get_balance address
```



### print_chain

> Print blocks in Radiancy. Actually a iterator to print every block stored in db.



__Example__

```
radiancy print_chain
```

### print_address

> Print a test address. I set the command to test transaction... lol~



__Example__

```
radiancy print_address
```



### send

> Transaction part. send RDC from current address to another.



__Example__

```
radiancy send <address> <amount>
```



### Contribute



udtrokia.



  



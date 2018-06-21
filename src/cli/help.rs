use cli::cli::CLI;

impl CLI {
    pub fn help(self) {
        println!("\n<-- Hello Yellow Brick Road -->");        
        println!("\nUsage: radiancy COMMAND;");
        println!("\nCOMMANDS:");
        println!("    create_blockchain         Generate a blockchain;");
        println!("    create_wallet             Generate a wallet;");
        println!("    get_balance               Get address balance;");
        println!("    print_chain               Print blocks in Radiancy;");
        println!("    print_address             Print blocks in Radiancy;");        
        println!("    send                      Send coin between addresses;");
        println!("\n<-- GoodBye Yellow Brick Road -->");
        println!("");
    }
}

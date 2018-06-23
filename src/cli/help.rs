use cli::cli::CLI;

impl CLI {
    pub fn help(self) {
        println!("\n<-- Hello Yellow Brick Road -->");        
        println!("\nUsage: radiancy COMMAND;");
        println!("\nCOMMANDS:");
        println!("    create_account            Generate an account in the current path;");
        println!("    create_blockchain         Generate a blockchain;");
        println!("    get_balance               Get the balance of the following address;");
        println!("    print_chain               Print blocks in Radiancy;");
        println!("    print_address             Print a test address;");   
        println!("    send                      Send RDC from current address to another.;");
        println!("\n<-- GoodBye Yellow Brick Road -->");
        println!("");
    }
}

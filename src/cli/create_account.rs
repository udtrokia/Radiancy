use cli::cli::CLI;
use wallet::utils::create_account;

impl CLI {
    pub fn _create_account(self) {
        println!("\ncreate_account.... {:?}\n", create_account());
    }
}

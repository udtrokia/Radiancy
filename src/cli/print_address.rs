pub use wallet::utils::print_address;
use cli::cli::CLI;

impl CLI {
    pub fn _print_address(self){
        print_address();
    }
}


mod alpha{
    pub mod beta{
        pub fn gema(){
            println!("Simple function" );
        }
    }
}

use alpha::beta::gema;

mod lib;
use lib::first::second::third;

use library_package::lib1::lib2::lib3;

fn main() {
gema();
third();
lib3();
}

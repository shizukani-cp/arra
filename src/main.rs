mod imd_lang_types;
mod builder;
use crate::builder::builder::generate_code;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "build".to_string(){
        let filename = &args[2];
        println!("{:?}", generate_code(&filename));
    } else if args[1] == "run".to_string(){

    } else {
        panic!("Sub command not found.");
    }
}

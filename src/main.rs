mod imd_lang_types;
mod builder;
use crate::builder::builder::generate_code;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "build".to_string(){
        let source_filename = &args[2];
        let imd_filename = &args[3];
        println!("{:?}", generate_code(&source_filename, &imd_filename));
    } else if args[1] == "run".to_string(){
        unimplemented!()
    } else {
        panic!("Sub command not found.");
    }
}

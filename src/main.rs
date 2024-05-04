mod imd_lang_types;
use imd_lang_types::ImdLangTypes;

mod compile;
use compile::compiler;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "compile".to_string(){
        let filename = &args[2];
        println!("{:?}", compiler::generate_code(&filename));
    } else if args[1] == "run".to_string(){

    } else {
        panic!("Sub command not found.");
    }
}

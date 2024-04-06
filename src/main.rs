use std::env;
use std::fs::File;
use std::io::prelude::*;

fn generate_code(filename:&String) -> Vec<Vec<String>> {
    // --snip--
    //println!("In file {}", filename);

    // ファイルが見つかりませんでした
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");
    let str_contents = contents.as_str();
    let mut code = vec![];
    for str_row in str_contents.split("\n"){
        let mut code_row = vec![];
        for cell in str_row.split(","){
            code_row.push(cell.trim().to_string());
        }
        code.push(code_row);
    }
    // テキストは\n{}です
    //println!("With text:\n{}", contents);
    return code;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{:?}", generate_code(&filename));
}
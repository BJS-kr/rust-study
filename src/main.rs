use std::fs::read_to_string;

mod parse;
mod run;


fn main() {
    let config = parse::Config::build().unwrap();
    let file_content = read_to_string(config.file_path).expect("file did not found");
    let results =run::run(&file_content, &config.query);

    for result in results {
        println!("{result}");
    }
}


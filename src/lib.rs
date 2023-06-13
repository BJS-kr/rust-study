pub mod parse;
pub mod run;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let config = parse::Config::build().unwrap();
        assert_eq!("./src/test.txt", config.file_path);
    }
}
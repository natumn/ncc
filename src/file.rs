extern crate clap;

use std::io::Read;
use self::clap::{App, Arg};
use std::fs::File;
use std::string::String;
use std::io::BufReader;
use std::path::Path;

pub fn new_file() -> Vec<char> {
    let matches = App::new("grc")
        .version("0.1.0")
        .author("natumn<natsume.pcuse75@gmail.com>")
        .about("garuda")
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Show version info"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input *.gr file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    let filename = matches.value_of("INPUT").unwrap();
    let string = String::from(filename);
    let path = Path::new(&string);
    let f = File::open(path).expect(&format!("Input file: {} doesn't exist!", filename));
    let mut br = BufReader::new(f);
    let mut s = String::new();
    br.read_to_string(&mut s).unwrap();
    s.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_file_test() {
        let test_file_content = new_file();
        let ans: Vec<char> = "let a = 4 + 5;".chars().collect();
        assert_eq!(ans, test_file_content);
    }
}

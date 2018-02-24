//pub use self::Stage::{Tokens, AST};

// #[derive(PartialEq, Clone, Debug)]
// pub enum Stage {
//    AST,
//    Token,
// }

mod file;

pub fn run_compiler() {
    let input = file::new_file();

    println!("{:?}", input);
    // let tokens = parseFile(input.as_str());
}

//fn parseFile(input: &str) {
//
//}

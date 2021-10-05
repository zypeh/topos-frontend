use bumpalo::{Bump, collections};

mod tokenising;
use tokenising::*;

mod parsing;
use parsing::*;

fn main() {
    use self::*;

    let text = "\"someText\"bind <> tighter < \n   fn \n\n newBlock";
    let bump = Bump::new();
    let mut token_stream = collections::Vec::new_in(&bump);
    Tokeniser::new(text).for_each(|x| token_stream.push(x));

    println!("{:?}", token_stream);
}

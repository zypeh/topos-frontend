use bumpalo::{Bump, collections};

mod tokenising;
use tokenising::*;

mod parsing;
use parsing::*;

fn main() {
    use self::*;

    let text = "f=apple";
    let bump = Bump::new();
    let mut token_stream = collections::Vec::new_in(&bump);
    Tokeniser::new(text).for_each(|x| token_stream.push(x));

    println!("Token stream: {:?}", token_stream);

    println!("AST will be: {:?}",
        Parser::new(token_stream).incrementally_build().incrementally_build().incrementally_build().state)
}

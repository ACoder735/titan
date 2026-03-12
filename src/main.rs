mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = String::from("x := 5 + 3");
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    
    let ast = parser.parse();
    println!("{:?}", ast);
}
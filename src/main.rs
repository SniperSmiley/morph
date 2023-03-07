//include the Lexer
use lexer::Lexer;
// This is the main file for the morph language
mod collection;
mod lexer;
fn main() {
    /// morph module define syntax changing tools in the morph language
    let code = collection::read_file("src\\example.mph");
    let mut lexer = Lexer::new(code);
    lexer.add_token_type("string".to_string(), r#"^"[^"]*""#.to_string());
    lexer.add_token_type("number".to_string(), r"^[0-9][0-9_]*".to_string());
    lexer.add_token_type("identifier".to_string(), r"^[a-zA-Z_][a-zA-Z0-9_]*".to_string());
    lexer.add_token_type("print".to_string(), r"^<<".to_string());
    lexer.add_token_type("whitespace".to_string(), r"^\s+".to_string());
    lexer.lex();
    for i in lexer.pending{
        println!("{}: {} {}", i.name, i.length, i.value);
    }
    /// syntax update module updates the syntax
    /// parser module takes the tokens and parses them into a tree
    /// interpreter module takes the tree and interprets it
    return ();
}
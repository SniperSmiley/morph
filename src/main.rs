// This is the main file for the morph language
mod collection;
mod lexer;
fn main() {
    /// morph module define syntax changing tools in the morph language
    let code = collection::read_file("src\\example.mph");
    lexer::lex(code);
    /// syntax update module updates the syntax
    /// parser module takes the tokens and parses them into a tree
    /// interpreter module takes the tree and interprets it
    return ();
}
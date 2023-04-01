mod collection;
#[derive(Clone,PartialEq)]
enum Syntax {
    RightHandSide, // a RHS is a vector of options that can be refferenced from an LHS
    LeftHandSide,  // a LHS is a name that can be resolved to a RHS
    Definition,    // a Def is a vector of options that must be matched in order
    Keyword,       // a Key is a string that must match the code exactly
    Repeat,        // a Rep is a argument that is used zero or more times aka * in regex
    Argument,      // a Arg is a argument that is used once or more times aka + in regex
    Option,        // a Opt is a argument that is used once or not at all aka ? in regex
    Choice,        // a Cho is a vector of options where only one must be matched
    Group,         // a Grp is a vector of options that must matched but can be in any order
}
#[derive(Clone)]
struct SxN{
    name:String,
    syntax:Syntax,
    options:Vec<SxN>,
}

fn rhs(name:&str,options:Vec<SxN>) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::RightHandSide,options}
}
fn lhs(name:&str) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::LeftHandSide,options:vec![]}
}
fn def(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Definition,options}
}
fn key(name:&str) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::Keyword,options:vec![]}
}
fn rep(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Repeat,options}
}
fn arg(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Argument,options}
}
fn opt(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Option,options}
}
fn cho(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Choice,options}
}
fn grp(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Group,options}
}
// need a struct that holds the AST the grammer and the code
struct AST{
    root: ASTNode,
}
#[derive(Clone)]
struct ASTNode{
    node_type: String,
    span: (usize,usize),
    children: Vec<ASTNode>,
}
impl ASTNode{
    fn new(node_type:String,span:(usize,usize),children:Vec<ASTNode>) -> ASTNode{
        ASTNode{node_type,span,children}
    }
}
struct Parser{
    grammar:Vec<SxN>,
    starting_point:SxN,
    code:String,
    ast:AST,
    current:usize,
}
impl Parser{
    fn rhs_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        for key in &key.options {
            match key.syntax {
                Syntax::RightHandSide => {
                    return false;
                },
                Syntax::LeftHandSide => {
                    if self.lhs_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Definition => {
                    if self.def_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Keyword => {
                    if self.key_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Repeat => {
                    if self.rep_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Argument => {
                    if self.arg_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Option => {
                    if self.opt_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Choice => {
                    if self.cho_check(&key,&ast_node){
                        return true;
                    }
                },
                Syntax::Group => {
                    if self.grp_check(&key,&ast_node){
                        return true;
                    }
                },
            }
        }
        false
    }
    fn lhs_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        // find the RHS with the name
        for rhs in &self.grammar {
            if rhs.name == key.name {
                // call a function that goes through the Def
                return self.rhs_check(&rhs.clone(),&ast_node);
            }
        }
        false
    }
    fn def_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        false
    }
    fn key_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        false
    }
    fn rep_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        false
    }
    fn arg_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        false
    }
    fn opt_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        false
    }
    fn cho_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        for key in &key.options {
            return false;
        }
        false
    }
    fn grp_check(&mut self,key:&SxN,ast_node:&ASTNode) -> bool{
        false
    }
}
// create a function that takes a RHS and the current position in the code and checks if the code matches a Def in the RHS
// create a function that goes through a Def and checks if the code matches staring at the current position in the code and if it does updates the AST and returns the new cursor position

fn main() {
    let code = collection::read_file("src\\example.mph");
    // create a syntax
    let test_eval = vec![
        rhs("Sum",     vec![cho(vec![lhs("Add"),lhs("Subtract"),lhs("Product")])]),
        rhs("Product", vec![cho(vec![lhs("Multiply"),lhs("Divide"),lhs("Atomic")])]),
        rhs("Atomic",  vec![cho(vec![lhs("Number"),def(vec![key("("),lhs("Sum"),key(")")])])]),
        rhs("Add",     vec![def(vec![lhs("Sum"),key("+"),lhs("Product")])]),
        rhs("Subtract",vec![def(vec![lhs("Sum"),key("-"),lhs("Product")])]),
        rhs("Multiply",vec![def(vec![lhs("Product"),key("*"),lhs("Atomic")])]),
        rhs("Divide",  vec![def(vec![lhs("Product"),key("/"),lhs("Atomic")])]),
        rhs("Number",  vec![opt(vec![key("-")]),arg(vec![lhs("Numeral")]),opt(vec![key("."),arg(vec![lhs("Numeral")])]),]),
        rhs("Numeral", vec![cho(vec![key("1"),key("2"),key("3"),key("4"),key("5"),key("6"),key("7"),key("8"),key("9"),key("0"),])]),
    ];
    let test_grammer = vec![
        rhs("PROGRAM",vec![def(vec![lhs("NUMBERS")])]),
        rhs("NUMERAL_WO_0",vec![cho(vec![
            key("1"),
            key("2"),
            key("3"),
            key("4"),
            key("5"),
            key("6"),
            key("7"),
            key("8"),
            key("9"),
        ])]),
        rhs("NUMERAL",vec![cho(vec![
            key("0"),
            lhs("NUMERAL_WO_0"),
        ])]),
        rhs("NUMBER",vec![def(vec![
            lhs("NUMERAL_WO_0"),
            rep(vec![lhs("NUMERAL")])
        ])]),
    ];

    let test_number = "1234567890".to_string();
    let ast_node = ASTNode{node_type:"".to_string(),span:(0,0),children:vec![]};
    let mut parser = Parser{
        grammar: test_grammer.clone(),
        starting_point: test_grammer[0].options[0].clone(),
        code: test_number,
        ast: AST{root:ast_node.clone()},
        current:0,
    };
    parser.rhs_check(&parser.starting_point.clone(),&ast_node);
}
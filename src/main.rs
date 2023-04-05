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
    Capture,       // a Cap is a argument that is captured
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
fn cap(name:&str,options:Vec<SxN>) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::Capture,options}
}
// need a struct that holds the AST the grammer and the code
#[derive(Clone)]
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
use std::collections::HashSet;
#[derive(Clone)]
struct Parser{
    grammar:Vec<SxN>,
    starting_point:SxN,
    code:String,
    ast:AST,
    current:usize,
    visited:HashSet<String>,
}
impl Parser{
    fn new(grammar:Vec<SxN>, starting_point:SxN, code:String) -> Parser{
        let ast = AST{root:ASTNode::new("".to_string(),(0,0),vec![])};
        let visited = HashSet::new();
        Parser{grammar,starting_point,code,ast,current:0, visited}
    }
    fn rhs_check(&mut self,key:&SxN,mut ast_node:&mut ASTNode) -> bool{
        for key in &key.options {
            match key.syntax {
                Syntax::RightHandSide => {
                    return false;
                },
                Syntax::LeftHandSide => {
                    if self.lhs_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Definition => {
                    if self.def_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Keyword => {
                    if self.key_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Repeat => {
                    if self.rep_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Argument => {
                    if self.arg_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Option => {
                    if self.opt_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Choice => {
                    if self.cho_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Group => {
                    if self.grp_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Capture => {
                    if self.cap_check(&key,&mut ast_node){
                        return true;
                    }
                },
            }
        }
        false
    }
    fn lhs_check(&mut self,key:&SxN,mut ast_node:&mut ASTNode) -> bool{
        // find the RHS with the name
        for rhs in &self.grammar {
            if rhs.name == key.name {
                // call a function that goes through the Def
                if self.visited.contains(&rhs.name) {
                    // The RHS has already been visited, so skip it
                    return false;
                } else {
                    self.visited.insert(rhs.name.clone());
                    return self.rhs_check(&rhs.clone(),&mut ast_node);
                }
            }
        }
        false
    }
    fn def_check(&mut self,key:&SxN,mut ast_node:&mut ASTNode) -> bool{
        let mut tester = self.clone();
        for key in &key.options {
            match key.syntax {
                Syntax::RightHandSide => {
                    return false;
                },
                Syntax::LeftHandSide => {
                    if !tester.lhs_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Definition => {
                    if !tester.def_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Keyword => {
                    if !tester.key_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Repeat => {
                    if !tester.rep_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Argument => {
                    if !tester.arg_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Option => {
                    if !tester.opt_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Choice => {
                    if !tester.cho_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Group => {
                    if !tester.grp_check(&key,&mut ast_node){
                        return false;
                    }
                },
                Syntax::Capture => {
                    if !tester.cap_check(&key,&mut ast_node){
                        return false;
                    }
                },
                
            }
        }
        *self = tester;
        true
    }
    fn key_check(&mut self,key:&SxN,ast_node:&mut ASTNode) -> bool{
        if self.code[self.current..].starts_with(&key.name){
            ast_node.span = (self.current,self.current+key.name.len());
            ast_node.node_type = key.name.clone();
            self.current += key.name.len();
            self.visited = HashSet::new();
            true
        }else{
            false
        }
    }
    fn rep_check(&mut self,key:&SxN,ast_node:&mut ASTNode) -> bool{
        while self.def_check(key, ast_node){}
        true
    }
    fn arg_check(&mut self,key:&SxN,ast_node:&mut ASTNode) -> bool{
        if self.def_check(key, ast_node){
            while self.def_check(key, ast_node){}
            return true;
        }
        false
    }
    fn opt_check(&mut self,key:&SxN,ast_node:&mut ASTNode) -> bool{
        self.def_check(key, ast_node);
        true
    }
    fn cho_check(&mut self,key:&SxN,mut ast_node:&mut ASTNode) -> bool{
        for key in &key.options {
            match key.syntax {
                Syntax::RightHandSide => {
                    return false;
                },
                Syntax::LeftHandSide => {
                    if self.lhs_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Definition => {
                    if self.def_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Keyword => {
                    if self.key_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Repeat => {
                    if self.rep_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Argument => {
                    if self.arg_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Option => {
                    if self.opt_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Choice => {
                    if self.cho_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Group => {
                    if self.grp_check(&key,&mut ast_node){
                        return true;
                    }
                },
                Syntax::Capture => {
                    if self.cap_check(&key,&mut ast_node){
                        return true;
                    }
                },
            }
        }
        false
    }
    fn grp_check(&mut self,key:&SxN,ast_node:&mut ASTNode) -> bool{
        false
    }
    fn cap_check(&mut self,key:&SxN,mut ast_node:&mut ASTNode) -> bool{
        let temp = self.current;
        if self.def_check(key,&mut ast_node){
            ast_node.span = (temp,self.current);
            return true;
        }
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
        rhs("Number",  vec![cap("number",vec![opt(vec![key("-")]),arg(vec![lhs("Numeral")]),opt(vec![key("."),arg(vec![lhs("Numeral")])]),])]),
        rhs("Numeral", vec![cho(vec![key("1"),key("2"),key("3"),key("4"),key("5"),key("6"),key("7"),key("8"),key("9"),key("0"),])]),
    ];
    let test_equation = "-1.5+2*3+1/5-5/11".to_string();
    let mut ast_equ = ASTNode{node_type:"".to_string(),span:(0,0),children:vec![]};
    let mut parser_equ = Parser{
        grammar: test_eval.clone(),
        starting_point: test_eval[0].options[0].clone(),
        code: test_equation.clone(),
        ast: AST{root:ast_equ.clone()},
        current:0,
        visited: HashSet::new(),
    };
    parser_equ.rhs_check(&parser_equ.starting_point.clone(),&mut ast_equ);
    println!("{}",parser_equ.current);

    let test_grammer = vec![
        rhs("PROGRAM",vec![def(vec![lhs("NUMBER")])]),
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
    let mut ast_node = ASTNode{node_type:"".to_string(),span:(0,0),children:vec![]};
    let mut parser = Parser{
        grammar: test_grammer.clone(),
        starting_point: test_grammer[0].options[0].clone(),
        code: test_number,
        ast: AST{root:ast_node.clone()},
        current:0,
        visited: HashSet::new(),
    };
    parser.rhs_check(&parser.starting_point.clone(),&mut ast_node);
    println!("{}",parser.current);
}
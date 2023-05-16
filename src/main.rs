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
    indent:usize,
}
impl Parser{
    fn new(grammar:Vec<SxN>, starting_point:SxN, code:String) -> Parser{
        let ast = AST{root:ASTNode::new("".to_string(),(0,0),vec![])};
        let visited = HashSet::new();
        Parser{grammar,starting_point,code,ast,current:0, visited, indent:0}
    }
    fn parse(&mut self){
        while self.current != self.code.len(){
            if let Some(ast_node) = self.rhs_check(&self.starting_point.clone()) {
                self.ast.root.children.push(ast_node);
            }
            else {
                break;
            }
        }
    }
    fn rhs_check(&mut self, in_key: &SxN) -> Option<ASTNode> {
        println!("{}rhs_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), in_key.name);
        self.indent+=1;
        let mut rhs_node = ASTNode::new(in_key.name.clone(),(self.current,self.current),vec![]);
        for key in &in_key.options {
            match key.syntax {
                Syntax::RightHandSide => return None,
                Syntax::LeftHandSide => {
                    if let Some(ast_node) = self.lhs_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Definition => {
                    if let Some(ast_node) = self.def_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Keyword => {
                    if let Some(ast_node) = self.key_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Repeat => {
                    if let Some(ast_node) = self.rep_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Argument => {
                    if let Some(ast_node) = self.arg_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Option => {
                    if let Some(ast_node) = self.opt_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Choice => {
                    if let Some(ast_node) = self.cho_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Group => {
                    if let Some(ast_node) = self.grp_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
                Syntax::Capture => {
                    if let Some(ast_node) = self.cap_check(&key) {
                        rhs_node.children.push(ast_node);
                    }
                },
            }
        }
        self.indent-=1;
        if rhs_node.children.len() == 0 {
            None
        } else {
            rhs_node.span.1 = self.current;
            Some(rhs_node)
        }
    }
    
    fn lhs_check(&mut self, key: &SxN) -> Option<ASTNode> {
        println!("{}lhs_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), key.name);
        self.indent+=1;
        // find the RHS with the name
        for rhs in &self.grammar {
            if rhs.name == key.name {
                // call a function that goes through the Def
                if self.visited.contains(&rhs.name) {
                    // The RHS has already been visited, so skip it
                    self.indent-=1;
                    return None;
                } else {
                    self.visited.insert(rhs.name.clone());
                    let tel = self.rhs_check(&rhs.clone());
                    self.indent-=1;
                    return tel;
                }
            }
        }
        self.indent-=1;
        None
    }
    
    fn def_check(&mut self, in_key: &SxN) -> Option<ASTNode> {
        println!("{}def_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), in_key.name);
        self.indent+=1;
        let start = self.current;
        let mut tester = self.clone();
        let mut def_node = ASTNode::new(in_key.name.clone(),(0,0),vec![]);
        for key in &in_key.options {
            match key.syntax {
                Syntax::RightHandSide => return None,
                Syntax::LeftHandSide => {
                    if let Some(ast_node) = tester.lhs_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Definition => {
                    if let Some(ast_node) = tester.def_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Keyword => {
                    if let Some(ast_node) = tester.key_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Repeat => {
                    if let Some(ast_node) = tester.rep_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Argument => {
                    if let Some(ast_node) = tester.arg_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Option => {
                    if let Some(ast_node) = tester.opt_check(&key) {
                        def_node.children.push(ast_node);
                    }
                },
                Syntax::Choice => {
                    if let Some(ast_node) = tester.cho_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Group => {
                    if let Some(ast_node) = tester.grp_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                Syntax::Capture => {
                    if let Some(ast_node) = tester.cap_check(&key) {
                        def_node.children.push(ast_node);
                    } else {
                        self.indent-=1;
                        return None;
                    }
                },
                
            }
        }
        *self = tester;
        def_node.span = (start, self.current);
        self.indent-=1;
        Some(def_node)
    }
    fn key_check(&mut self,key:&SxN) -> Option<ASTNode>{
        println!("{}key_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), key.name);
        //println!("          {} ", self.code[self.current..self.current+key.name.len()].to_string());
        if self.code[self.current..].starts_with(&key.name){
            let ast_node = ASTNode::new(key.name.clone(),(self.current,self.current+key.name.len()),vec![]);
            self.current += key.name.len();
            self.visited = HashSet::new();
            return Some(ast_node);
        }else{
            return None;
        }
    }
    fn rep_check(&mut self, key: &SxN) -> Option<ASTNode> {
        println!("{}rep_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), key.name);
        self.indent+=1;
        let start = self.current;
        if let Some(def_ast_node) = self.def_check(key) {
            let mut ast_node = ASTNode::new(key.name.clone(), (0, 0), vec![def_ast_node.clone()]);
            while let Some(ast_node2) = self.def_check(key) {
                ast_node.children.push(ast_node2.clone());
            }
            ast_node.span = (start, self.current);
            self.indent-=1;
            Some(ast_node)
        } else {
            self.indent-=1;
            None
        }
    }
    
    fn arg_check(&mut self,key:&SxN) -> Option<ASTNode>{
        println!("{}arg_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), key.name);
        self.indent+=1;
        let start = self.current;
        match self.def_check(key){
            Some(def_ast_node) => {
                let mut ast_node = ASTNode::new(key.name.clone(),(0,0),vec![def_ast_node.clone()]);
                let mut going = true;
                while going{
                    match self.def_check(key){
                        Some(ast_node2) => {
                            ast_node.children.push(ast_node2.clone());

                        },
                        None => {
                            ast_node.span = (start,self.current);
                            going = false;
                        },
                    }
                }
                self.indent-=1;
                return Some(ast_node);
            },
            None => {
                self.indent-=1;
                return None
            }
        }
    }
    fn opt_check(&mut self,key:&SxN) -> Option<ASTNode>{
        println!("{}opt_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), key.name);
        self.indent+=1;
        let tel = self.def_check(key);
        self.indent-=1;
        return tel;
    }
    fn cho_check(&mut self,key:&SxN) -> Option<ASTNode>{
        println!("{}cho_check({})", (0..self.indent).map(|_|"  ").collect::<String>(), key.name);
        self.indent+=1;
        for key in &key.options {
            match key.syntax {
                Syntax::RightHandSide => {
                    self.indent-=1;
                    return None;
                },
                Syntax::LeftHandSide => {
                    if let Some(node) = self.lhs_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
                Syntax::Definition => {
                    if let Some(node) = self.def_check(&key){
                        self.indent-=1;
                        return Some(node)
                    }
                },
                Syntax::Keyword => {
                    if let Some(node) = self.key_check(&key){
                        self.indent-=1;
                        return Some(node)
                    }
                },
                Syntax::Repeat => {
                    if let Some(node) = self.rep_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
                Syntax::Argument => {
                    if let Some(node) = self.arg_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
                Syntax::Option => {
                    if let Some(node) = self.opt_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
                Syntax::Choice => {
                    if let Some(node) = self.cho_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
                Syntax::Group => {
                    if let Some(node) = self.grp_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
                Syntax::Capture => {
                    if let Some(node) = self.cap_check(&key){
                        self.indent-=1;
                        return Some(node);
                    }
                },
            }
        }
        self.indent-=1;
        None
    }
    fn grp_check(&mut self,key:&SxN) -> Option<ASTNode>{
        None
    }
    fn cap_check(&mut self,key:&SxN) -> Option<ASTNode>{
        println!("cap_check({})", key.name);
        let temp = self.current;
        if let Some(mut node) = self.def_check(key){
            println!("\t{}",self.code[temp..self.current].to_string());
            node.span = (temp,self.current);
            return Some(node);
        }
        None
    }
}
// create a function that takes a RHS and the current position in the code and checks if the code matches a Def in the RHS
// create a function that goes through a Def and checks if the code matches staring at the current position in the code and if it does updates the AST and returns the new cursor position

fn main() {
    let code = collection::read_file("src\\example.mph");
    // create a syntax
    let test_eval = vec![
        rhs("Val",     vec![def(vec![key("val "),lhs("Name"),rep(vec![key(" ")]),key("="),rep(vec![key(" ")]),lhs("Number"),key(";")])]),
        rhs("Eval",    vec![def(vec![rep(vec![lhs("Sum")])])]),
        rhs("Name",    vec![def(vec![lhs("Letter"),rep(vec![cho(vec![lhs("Letter"),key("_")])])])]),
        rhs("Letter",  vec![cho(vec![key("a"),key("b"),key("c"),key("d"),key("e"),key("f"),key("g"),key("h"),key("i"),key("j"),key("k"),key("l"),key("m"),key("n"),key("o"),key("p"),key("q"),key("r"),key("s"),key("t"),key("u"),key("v"),key("w"),key("x"),key("y"),key("z")])]),
        rhs("Number",  vec![def(vec![opt(vec![key("-")]),arg(vec![lhs("Numeral")]),opt(vec![key("."),arg(vec![lhs("Numeral")])]),])]),
        rhs("Numeral", vec![cho(vec![key("1"),key("2"),key("3"),key("4"),key("5"),key("6"),key("7"),key("8"),key("9"),key("0"),])]),
    ];
    let test_equation = "val west = 3;".to_string();
    let mut ast_equ = ASTNode{node_type:"".to_string(),span:(0,0),children:vec![]};
    let mut parser_equ = Parser{
        grammar: test_eval.clone(),
        starting_point: test_eval[0].clone(),
        code: test_equation.clone(),
        ast: AST{root:ast_equ.clone()},
        current:0,
        visited: HashSet::new(),
        indent:0,
    };
    parser_equ.rhs_check(&test_eval[0].clone());
    parser_equ.parse();
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
        indent:0,
    };
    //parser.rhs_check(&parser.starting_point.clone());
    //println!("{}",parser.current);
}
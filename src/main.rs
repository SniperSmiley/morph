mod collection;
#[derive(PartialEq)]
enum Syntax {
    RightHandSide, // a RHS is a vector of options that can be refferenced from an LHS
    LeftHandSide,  // a LHS is a name that can be resolved to a RHS
    Definition,    // a Def is a vector of options that must be matched in order
    Keyword,       // a Key is a string that must match the code exactly
    Repeat,        // a Rep is a argument that is used zero or more times aka * in regex
    Argument,      // a Arg is a argument that is used once or more times aka + in regex
    Option,        // a Opt is a argument that is used once or not at all aka ? in regex
    Choice,        // a Cho is a vector of options where only one must be matched
    Group,
}
struct SxN{
    name:String,
    syntax:Syntax,
    options:Vec<SxN>,
}

fn RHS(name:&str,options:Vec<SxN>) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::RightHandSide,options}
}
fn LHS(name:&str) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::LeftHandSide,options:vec![]}
}
fn Def(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Definition,options}
}
fn Key(name:&str) -> SxN{
    SxN{name:name.to_string(),syntax:Syntax::Keyword,options:vec![]}
}
fn Arg(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Argument,options}
}
fn Opt(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Option,options}
}
fn Rep(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Repeat,options}
}
fn Grp(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Group,options}
}
fn Cho(options:Vec<SxN>) -> SxN{
    SxN{name:"".to_string(),syntax:Syntax::Choice,options}
}
fn ll(grammar:&Vec<SxN>,definition:&Vec<SxN>,code:String,current:usize,ast_node:ASTNode) -> bool{
    for key in definition {
        if key.syntax == Syntax::LeftHandSide {
            // find the RHS with the name
            for rhs in grammar {
                if rhs.name == key.name {
                    // call a function that goes through the Def
                    if ll(grammar,&rhs.options,code,current,ast_node) {
                        // if the code matches the Def then return true
                        return true;
                    } else {
                        // if the code does not match the Def then return false
                        return false;
                    }
                }
            }
        }
    }
    false
}
fn first_do_the_thing(grammar:Vec<SxN>,code:String){   
    // starting with the first RHS in the grammar go through each option
    let current = 0;
    for option in &grammar[0].options {
        let mut test = current.clone();
        if option.options.len() == 0{
            if Syntax::LeftHandSide == option.syntax {
                // go into grammar and find the RHS with the name
                for rhs in &grammar {
                    if rhs.name == option.name {
                        // call a function that will go through the RHS to see if there is a matching Def
                    }
                }
            }
            // if the option is a keyword then check if the code contians the keyword
            else if Syntax::Keyword == option.syntax {
                if (code.len() > test + option.name.len()) && code[test..test+option.name.len()] == option.name {
                    // if the keyword is found then move the current position to the end of the keyword
                    test += option.name.len();
                } else {
                    // if the keyword is not found then the code does not match the grammar
                    return ();
                }
            } else {

            }
        }
    }
}
struct AST{
    root: ASTNode,
}
struct ASTNode{
    node_type: String,
    span: (usize,usize),
    children: Vec<ASTNode>,
}
// create a function that takes a grammar and some code and returns a AST
fn grammar_to_AST(grammar:Vec<SxN>,code:String) -> AST{
    // create a AST
    let AST_ = ASTNode{node_type:"".to_string(),span:(0,0),children:vec![]};
    // go through the grammar and create a AST

    // return the AST
    let ast = AST{root:AST_};
    ast
}
// create a function that takes a RHS and the current position in the code and checks if the code matches a Def in the RHS
// create a function that goes through a Def and checks if the code matches staring at the current position in the code and if it does updates the AST and returns the new cursor position

fn main() {
    let code = collection::read_file("src\\example.mph");
    // create a syntax
    /*let _gammars = vec![
        RHS{name:"PROGRAM".to_string(),options:vec![LHS("COMPSTMT".to_string())]},
        RHS{name:"T".to_string(),options:vec![Key(";".to_string()),Key("\n".to_string()),]},
        RHS{name:"COMPSTMT".to_string(),options:vec![
            Def(vec![
                LHS("STMT".to_string()),
                Rep(vec![LHS("T".to_string()),LHS("EXPR".to_string())]),
                Opt(vec![LHS("T".to_string())])
            ]),
        ]},
        RHS{name:"STMT".to_string(),options:vec![
            Def(vec![
                LHS("CALL".to_string()),
                Key("do".to_string()),
                Opt(vec![
                    Key("|".to_string()),
                    Arg(vec![LHS("BLOCK_VAL".to_string())]),
                    Key("|".to_string())
                ]),
                LHS("COMPSTMT".to_string()),
                Key("end".to_string()),
            ]),
            Def(vec![
                Key("undef".to_string()),
                LHS("FNAME".to_string())
            ]),
            Def(vec![
                Key("alias".to_string()),
                LHS("FNAME".to_string()),
                LHS("FNAME".to_string())
            ]),
        ]}
    ];*/
    let test_eval = vec![
        RHS("Sum",     vec![Cho(vec![LHS("Add"),LHS("Subtract"),LHS("Product")])]),
        RHS("Product", vec![Cho(vec![LHS("Multiply"),LHS("Divide"),LHS("Atomic")])]),
        RHS("Atomic",  vec![Cho(vec![LHS("Number"),Def(vec![Key("("),LHS("Sum"),Key(")")])])]),
        RHS("Add",     vec![Def(vec![LHS("Sum"),Key("+"),LHS("Product")])]),
        RHS("Subtract",vec![Def(vec![LHS("Sum"),Key("-"),LHS("Product")])]),
        RHS("Multiply",vec![Def(vec![LHS("Product"),Key("*"),LHS("Atomic")])]),
        RHS("Divide",  vec![Def(vec![LHS("Product"),Key("/"),LHS("Atomic")])]),
        RHS("Number",  vec![Opt(vec![Key("-")]),Arg(vec![LHS("Numeral")]),Opt(vec![Key("."),Arg(vec![LHS("Numeral")])]),]),
        RHS("Numeral", vec![Opt(vec![Key("1"),Key("2"),Key("3"),Key("4"),Key("5"),Key("6"),Key("7"),Key("8"),Key("9"),Key("0"),])]),
    ];
    let test_grammer = vec![
        RHS("PROGRAM",vec![Def(vec![LHS("NUMBERS")])]),
        RHS("NUMERAL_WO_0",vec![Def(vec![
            Key("1"),
            Key("2"),
            Key("3"),
            Key("4"),
            Key("5"),
            Key("6"),
            Key("7"),
            Key("8"),
            Key("9"),
        ])]),
        RHS("NUMERAL",vec![Def(vec![
            Key("0"),
            LHS("NUMERAL_WO_0"),
        ])]),
        RHS("NUMBER",vec![Def(vec![
            LHS("NUMERAL_WO_0"),
            Rep(vec![LHS("NUMERAL")])
        ])]),
    ];

    let test_number = "1234567890".to_string();

    
}
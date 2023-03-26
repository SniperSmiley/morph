mod collection;
#[derive(PartialEq)]
enum Syntax {
    RightHandSide,
    LeftHandSide,
    Definition,
    Keyword,
    Argument,
    Option,
    Repeat,
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
fn first_do_the_thing(grammar:Vec<SxN>,code:String){   
    // starting with the first RHS in the grammar go through each option
    let current = 0;
    for option in grammar[0].options {
        let name = option.name.clone();
        let syntax = option.syntax.clone();
        let options = option.options.clone();
        let test= current.clone();
        if options.len() == 0{
            if Syntax::LeftHandSide == syntax {
                // go into grammar and find the RHS with the name
                for rhs in grammar {
                    if rhs.name == name {
                        // call a function that will go through the RHS to see if there is a matching Def
                    }
                }
            }
            // if the option is a keyword then check if the code contians the keyword
            else if Syntax::Keyword == option.syntax {
                let name = option.name.clone();
                if code.len() > test + name.len() && code[test..test+name.len()] == name {
                    // if the keyword is found then move the current position to the end of the keyword
                    test += name.len();
                } else {
                    // if the keyword is not found then the code does not match the grammar
                    return false;
                }
            } else {

            }
        }
        do_the_thing(option,0,code);
    }
}
// create a function that takes a grammar and some code and returns a AST
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
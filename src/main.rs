use std::fs;
use std::env;
use std::io::prelude::*;
mod lexer;
fn main() {
    let input = "a b 1 \"hello world!\" / this is a comment\n";

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    //read from a file to add transitions 
    let mut contents = fs::read_to_string("src\\transitions.txt").expect("Something went wrong reading the file");
    let mut fsm = lexer::FSM::new();
    println!("{}",contents);
    //go through the file and add transitions
    for line in contents.lines() {
        let line = line.trim();
        let mut iter = line.split_whitespace();
        let state = iter.next().unwrap();
        let trigger = iter.next().unwrap();
        let next_mode = iter.next().unwrap_or("None");
        let token_type = iter.next().unwrap_or("None");
        //create a transition
        let transition = lexer::Transition {
            trigger: trigger.chars().next().unwrap(),
            next_mode: if next_mode == "None" { None } else { Some(next_mode.to_string()) },
            token_type: if token_type == "None" { None } else { Some(token_type.to_string()) },
        };
        fsm.add_transition(state.to_string(), transition);
    }
    println!("tokens");
    let tokens = fsm.lex(input);
    for token in tokens {
        println!("{} {}", token.lexeme,token.token_type);
    }
}

use std::collections::HashMap;
use std::hash;

pub struct Token {
    pub token_type: String,
    pub lexeme: String,
}

pub struct FSM {
    // The current mode of the FSM
    current_mode: String,

    // A mapping from modes to sets of transitions
    transitions: HashMap<String, Vec<Transition>>,
}

pub struct Transition {
    // The character that triggers this transition
    pub trigger: char,

    // The mode to transition to, if any
    pub next_mode: Option<String>,

    // The token type to emit, if any
    pub token_type: Option<String>,
}

impl FSM {
    pub fn new() -> Self {
        // Create an initial mode with some transitions
        let mut fsm = FSM {
            current_mode: "normal".to_string(),
            transitions: HashMap::new(),
        };
        fsm.transitions.insert("normal".to_string(), vec![]);
        // Add more modes and transitions as needed...
        fsm
    }

    pub fn add_transition(&mut self, mode: String, transition: Transition) {
        self.transitions.entry(mode).or_default().push(transition);
    }

    pub fn remove_transition(&mut self, mode: String, trigger: char) {
        if let Some(transitions) = self.transitions.get_mut(&mode) {
            transitions.retain(|t| t.trigger != trigger);
        }
    }

    pub fn lex(&mut self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lexeme = String::new();
        let mut current_token_type = String::new();
    
        for c in input.chars() {
            let transitions = match self.transitions.get(&self.current_mode) {
                Some(transitions) => transitions,
                None => {
                    // No transitions are defined for this mode, so treat this character as a normal token
                    lexeme.push(c);
                    continue;
                }
            };
            let transition = transitions.iter().find(|t| t.trigger == c);
            
            match transition {
                Some(t) => {
                    // A transition was found for this character
                    tokens.push(Token { token_type:self.current_mode.clone(), lexeme: lexeme.clone() });
                    lexeme.clear();
                    if let Some(token_type) = t.token_type.clone() {
                        current_token_type = token_type;
                    }
                    if let Some(next_mode) = t.next_mode.clone() {
                        self.current_mode = next_mode;
                    }
                }
                None => {
                    // No transition was found for this character
                    
                }
            }
            lexeme.push(c);
        }
    
        // Emit a token for any remaining lexeme
        if !lexeme.is_empty() {
            tokens.push(Token {
                token_type: self.current_mode.clone(),
                lexeme,
            });
        }
        tokens
    }         
}
use pcre2::bytes::Regex;
/// lexer module reads to the next syntax change
pub struct Token{
    pub name: String,
    pub length: usize,
    pub value: String,
}
impl Token{
    pub fn len(&self) -> usize{
        return self.length.clone();
    }
}
struct TokenType{
    name: String,
    regex: Regex,
}
impl TokenType{
    pub fn capture(&self, window: &str) -> Option<Token>{
        if let Some(capture) = self.regex.captures(window.as_bytes()).unwrap(){
            if true{
                return Some(Token{
                    name: self.name.clone(),
                    length: capture.get(0).unwrap().end() - capture.get(0).unwrap().start(),
                    value: String::from_utf8(capture.get(0).unwrap().as_bytes().to_vec()).unwrap(),
                });
            }
        }
        return None;
    }
}
pub struct Lexer{
    code: String,
    location: usize,
    pub pending: Vec<Token>,
    token_types: Vec<TokenType>,
}
impl Lexer{
    pub fn new(code: String) -> Lexer{
        return Lexer{
            code: code,
            location: 0,
            pending: Vec::new(),
            token_types: Vec::new(),
        };
    }
    pub fn add_token_type(&mut self, name: String, regex: String){
        self.token_types.push(TokenType{
            name: name,
            regex: Regex::new(regex.as_str()).unwrap(),
        });
    }
    pub fn window(&self) -> String{
        return self.code[self.location..].to_string();
    }

    pub fn consume_character(&mut self){
        let mut potential:Token = Token{
            name: "unknown".to_string(),
            length: 0,
            value: "".to_string(),
        };
        for token_type in &self.token_types{
            if let Some(token) = token_type.capture(self.window().as_str()){
                if token.len() > potential.len(){
                    potential = token;
                }
            }
        }
        if potential.len() > 0{
            self.location += potential.len();
            self.pending.push(potential);
        }else{
            self.pending.push(Token{
                name: "unknown".to_string(),
                length: 1,
                value: self.code[self.location..self.location+1].to_string(),
            });
            self.location += 1;
        }
    }
    pub fn lex(&mut self){
        while self.location < self.code.len(){
            self.consume_character();
        }
    }
}
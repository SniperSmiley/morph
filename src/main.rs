mod collection;

struct ParserExpressionGrammer {

}

impl ParserExpressionGrammer {
    pub fn new() -> ParserExpressionGrammer {
        return ParserExpressionGrammer {

        };
    }
}

struct Interpreter {
    parser: ParserExpressionGrammer,
    functions: Vec<Function>,
}
impl Interpreter {
    pub fn new(parser: ParserExpressionGrammer) -> Interpreter {
        return Interpreter {
            parser: parser,
            functions: Vec::new(),
        };
    }
    pub fn add_function(&mut self, name: &str, function: Function) -> {
        self.functions.push(function);
    }
    pub fn run(&mut self, code: String) {
        return ();
    }
}

struct ASTNode {
    name: String,
    value: String,
    children: Vec<ASTNode>,
}

fn main() {
    let code = collection::read_file("src\\example.mph");
    // create a mutatable core parser that can start parsing the code
    let mut coreParser = ParserExpressionGrammer::new();
    //coreParser.extend("extend: 'extend' IDENTIFIER? '{' (STRING,)* '}'");
    // 
    // add a function to the interpreter
    let mut interpreter = Interpreter::new(coreParser);
    interpreter.add_function("print", |args| {
        println!("{}", args[0]);
        return Value::Null;
    });
    // run the interpreter
    interpreter.run(code);

}
use pcre2::bytes::Regex;
/// lexer module reads to the next syntax change
struct pattern{
    name: String,
    match_: Regex,
    begin: Regex,
    end: Regex
}

pub fn lex(code: String){
    //let patterns = vec![];
    for line in code.lines(){
        let re = Regex::new(r"<<(.*?(?=<<|\Z|\||;))").unwrap();
        let mut captured = false;
        for result in re.captures_iter(&line.as_bytes()){
            let t = result.unwrap();
            let m = t.get(1).unwrap();
            let s = m.as_bytes().to_vec().into_iter().map(|x| x as char).collect::<String>();
            print!("{} ", s);
            captured = true;
        }
        if captured{
            println!();
        }
    }
}
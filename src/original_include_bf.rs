


fn main() {
  //a feature, a syntax, a document, data, a structure, a paradigm, a program

  //example implementing brainfuck
 "global variable-level mutation operation '>'(){
    global ptr
    ptr++
  }
  global variable-level mutation operation '<'(){
    global ptr
    ptr--
  }
  global variable-level mutation operation '+'(){
    global array
    global ptr
    array[ptr]++
  }
  global variable-level mutation operation '-'(){
    global array
    global ptr
    array[ptr]--
  }
  global console mutation operation '.'(){
    global console
    global array
    global ptr
    global char
    console<<chr(array[ptr])
  }
  global scope operation '['(){
    global array
    global ptr
    global hierarchy
    hierarchy add scope new loop start with condition(array[ptr])
  }
  global scope operation ']'(){
    global hierarchy
    hierarchy add scope new loop end
  }
    "
  //the line is split into fields and polarity
  let mut field_modifiers=vec!["print","oil"];
  script = "print()".to_string().split('\n');
  for line in script {

  }

}

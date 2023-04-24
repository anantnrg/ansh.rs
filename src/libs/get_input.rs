use std::io::stdin;

pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input;
}

 use std::io;

fn get_non_alphabets(string: &str) -> (Vec<usize>, Vec<usize>){
    let mut spaces = vec![0]; // includes the postion of the first letter (first word)
    let mut n_alph = vec![];
    for (i, item) in string.chars().enumerate() {
        if item ==  ' '{
            if spaces.iter().any(|&j| j!=i) {
                spaces.push(i)
            }
        } else if item.is_alphabetic() == false {
            n_alph.push(i);
        }
    }
    (spaces, n_alph)
} 

fn get_slices(string: &str, mut spaces: Vec<usize>) -> Vec<&str> {
    let mut slices: Vec<&str> = vec![];
    spaces.push(string.len());
    let len = spaces.len();
    for (i, &item) in spaces[..len-1].iter().enumerate() {
        println!("{i}");
        if i == 0 {
            slices.push(&string[item..spaces[i+1]])
        } else {
            slices.push(&string[item+1..spaces[i+1]])
        }
    }
    slices
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let input = input.trim(); // get rid of \n
    // let bytes = input.as_bytes();
    let (spaces, n_alph) = get_non_alphabets(input);
    println!("{:?}", spaces);
    let charnum = input.len() as f64; // number of characters
    let spacenum = if spaces[0] == 0 {
            (spaces.len() - 1) as f64 // excludes the case that the first character is a space
        } else {
            spaces.len() as f64
        };
    let n_alphnum = n_alph.len() as f64; // number of non-alphabetic characters
    //letters per word = (number of character - number of spaces - number of non-alphabets) / number of words
    let lpw: f64 = (charnum - spacenum - n_alphnum)/(spaces.len() as f64);
    println!("Letters per word: {}", lpw);
    let slices = get_slices(input, spaces);
    println!("{:?}", slices)
}
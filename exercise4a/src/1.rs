use itertools::Itertools;
use std::collections::BTreeMap;
use std::io;

fn analyzer(string: &str) -> (){
    let mut letters = vec![];
    let mut vowels:Vec<usize> = vec![0; 8]; // amount of the chars ['a', 'e', 'i', 'o', 'u', 'ö', 'ä', 'ü'];
    for (i, item) in string.chars().enumerate() {
        match item {
            'a' => vowels[0] += 1, 
            'e' => vowels[1] += 1, 
            'i' => vowels[2] += 1, 
            'o' => vowels[3] += 1, 
            'u' => vowels[4] += 1, 
            'ö' => vowels[5] += 1, 
            'ä' => vowels[6] += 1, 
            'ü' => vowels[7] += 1,
            _ => ()
        }
        if item. is_alphabetic(){
            letters.push(i);
        }
        }
    println!("['a', 'e', 'i', 'o', 'u', 'ö', 'ä', 'ü'] = {:?}", vowels);
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let input = input.trim(); // get rid of \n

    for word in input.split_whitespace(){
        println!("{:?}:", word);
        analyzer(word);
    }

    // functional programming
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'ö', 'ä', 'ü'];
    for word in input.split_whitespace(){
        let result: BTreeMap<char, u32> = word
            .to_lowercase()
            .chars()
            .filter(|char| vowels.contains(char))
            .chunk_by(|&char| char)
            .into_iter()
            .map(|(char, l)| (char, l.count() as u32))
            .collect();
        println!("{:?}", result);
    }
}
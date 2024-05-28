fn main() {
    let lecture: Option<String> = Some("INF-B-240".to_string());
    println!("Adress of lecture (before): {:p}", &lecture);
    match lecture {
        Some(ref i) => println!("{}", i),
        _ => println!("None")
    }
    println!("Adress of lecture (after): {:p}", &lecture);
    println!(
        "I'm attending the module {}",
        lecture.unwrap_or("programming".into())
    )
}
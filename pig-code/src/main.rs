use std::io;

fn main() {
    println!("Pig code!");

    println!("Please input your word.");

    let mut word = String::new();

    let consonants:Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    let mut count = 0;

    for c in word.chars() {
        if count == 1 { break; }
        let target = c;
        if consonants.contains(&target) {
            let suffix = String::from("-hay");
            let output = word.to_owned() + &suffix;
            println!("OUTPUT {}", output);
        } else {
            let first_char = &word[0..1];
            let suffix = first_char.to_owned() + &String::from("ay");
            let divider = String::from("-");
            let sliced_word = &word[1..];
            let output = sliced_word.to_owned() + &divider + &suffix;
            println!("OUTPUT {output}");
        }
        count += 1;
    }
}

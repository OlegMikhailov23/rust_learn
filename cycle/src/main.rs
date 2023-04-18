fn main() {
    let start = 1;
    let end = 1000 * 1000 * 1000;
    for number in (start..end).rev() {
        println!("{number}!")
    }

    println!("End")
}

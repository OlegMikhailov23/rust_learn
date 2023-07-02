use std::io;

fn main() {
    println!("Welcome Fibo calculator!");

    loop {
        println!("Insert fibo digit!");

        let mut fibo = String::new();

        io::stdin()
            .read_line(&mut fibo)
            .expect("Failed to read line");

        let n: u32 = match fibo.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut a:i64 = 1;
        let mut b:i64 = 1;

        for _number in 3..n + 1 {
            let c = a + b;
            a = b;
            b = c;
        }

        println!("Your fibo sum is {b}! BYU :)");
        break;
    }
}

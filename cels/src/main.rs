use std::io;

fn main() {
    println!("Welcome to Farenheit converter!");
    loop {
        println!("Please input temperature in forenheit.");
        let mut input_farenheit = String::new();

        io::stdin().read_line(&mut input_farenheit).expect("Failed to read line");

        let input_farenheit: f64 = match input_farenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You should insert only digit :(");
                continue
            },
        };

        println!("You insert: {input_farenheit}");
        let result: f64 = (input_farenheit - 32.0) * 5.0 / 9.0;
        println!("{} in Farenhrit equal {} in Cels", input_farenheit, result);

        println!("Buy buy :)");
        break;
    }
}

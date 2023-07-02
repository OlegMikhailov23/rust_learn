use std::collections::HashMap;

fn main() {
    let mut list: Vec<u32> = vec![100, 23, 33, 7, 9, 36, 36, 105, 99, 45, 36];

    let mut sum: u32 = 0;

    let vec_length: u32 = list.len().try_into().unwrap();

    {
        for i in list.iter() {
            sum += i;
            println!("{}", i);
        }
        let avarage = sum / vec_length;
        println!("Avarege is: {avarage}");
    }

    {
        list.sort();
        let idx:usize = (vec_length / 2).try_into().unwrap();

        let median = list[idx];
        println!("Median is: {median}");
    }

    {
        let mut digits = HashMap::new();

        for digit in list.iter() {
            let count = digits.entry(digit).or_insert(0);
            *count += 1;
        }

        let mut results: Vec<i32> = Vec::new();
        for digit in digits.iter() {
            results.push(*digit.1);
        }
        results.sort();
        let times: &i32 = results.last().unwrap();
        println!("Mod is {}", times);
    }
}

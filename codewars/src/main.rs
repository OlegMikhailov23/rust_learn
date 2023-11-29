fn main() {
    grow([1,2,3,5,7,8,9].to_vec());
    let alt_string = to_alternating_case("hello WORLD");
    println!("alternative string: {}", alt_string);
    let powers_of_two_res = powers_of_two(37);
    println!("powers of two: {:?}", powers_of_two_res);
}

fn grow(nums: Vec<i32>) -> i32 {
    let mut summ: i32 = 1;
    for num in nums {
        summ = summ * num;
    }
    println!("{}", summ);
    return summ;
}

fn to_alternating_case(s: &str) -> String {
  let result = s.chars().map(|c| {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c
    }
  }).collect();

  result
}

fn powers_of_two(n: u8) -> Vec<u128> {
    let mut res:Vec<u128> = Vec::new();
    let exp: i32 = 2;

    for i in 0..= n {
        res.push(exp.pow(i.into()).try_into().unwrap());
        println!("{}", i)
    }

    res
}

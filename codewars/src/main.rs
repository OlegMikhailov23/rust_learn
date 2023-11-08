fn main() {
    grow([1,2,3,5,7,8,9].to_vec());
}

fn grow(nums: Vec<i32>) -> i32 {
    let mut summ: i32 = 1;
    for num in nums {
        summ = summ * num;
    }
    println!("{}", summ);
    return summ;
}

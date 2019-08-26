pub fn problem_one(number: i32)-> i32{
    let mut count: i32 = 0;
    for sub_number in 0..number{
        if (sub_number % 3) == 0 || (sub_number % 5) == 0{
            count += sub_number;
        }
    }
    return count;
}
mod set_one;

fn main() {
    let p_1 : i32 = set_one::problem_one(1000);
    let p_2 : u32 = set_one::problem_two(4000000);
    let p_3 : u64 = set_one::problem_three(600851475143);
    let p_4 : u32 = set_one::problem_four();

    println!("Problem 001: {}", p_1);
    println!("Problem 002: {}", p_2);
    println!("Problem 003: {}", p_3);
    println!("Problem 004: {}", p_4);
}

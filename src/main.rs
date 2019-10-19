mod set_one;

fn main() {
    let p_1 : i32 = set_one::problem_one(1000);
    let p_2 : u32 = set_one::problem_two(4000000);
    let p_3 : u64 = set_one::problem_three(600851475143);
    println!("Problem One: {}", p_1);
    println!("Problem Two: {}", p_2);
    println!("Problem Three: {}", p_3);
}

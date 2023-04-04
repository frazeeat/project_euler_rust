mod set_one;

fn main() {
    let problem_eight_input =  String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");

    let p_1 : i32 = set_one::problem_one(1000);
    let p_2 : u32 = set_one::problem_two(4000000);
    let p_3 : u64 = set_one::problem_three(600851475143);
    let p_4 : u32 = set_one::problem_four();
    let p_5 : u32 = set_one::problem_five(20);
    let p_6 : u32 = set_one::problem_six(100);
    let p_7 : u64 = set_one::problem_seven(10001);
    let p_8 : u128 = set_one::problem_eight(problem_eight_input, 13);
    let p_9 : i32 = set_one::problem_nine(1000);
    let p_10 : u128 = set_one::problem_ten(2000000);

    println!("Problem 001: {}", p_1);
    println!("Problem 002: {}", p_2);
    println!("Problem 003: {}", p_3);
    println!("Problem 004: {}", p_4);
    println!("Problem 005: {}", p_5);
    println!("Problem 006: {}", p_6);
    println!("Problem 007: {}", p_7);
    println!("Problem 008: {}", p_8);
    println!("Problem 009: {}", p_9);
    println!("Problem 010: {}", p_10);
}

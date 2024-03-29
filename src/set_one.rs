struct Fibonacci{
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci{
    type Item = u32;

    fn next(&mut self)-> Option<u32>{
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci{curr: 1, next: 1}
}

pub fn problem_one(number: i32)-> i32{
    let mut count: i32 = 0;
    for sub_number in 0..number{
        if (sub_number % 3) == 0 || (sub_number % 5) == 0{
            count += sub_number;
        }
    }
    return count;
}

pub fn problem_two(limit: u32)->u32{
    let mut count: u32 = 0;
    let mut fib = fibonacci();
    while let Some(i) = fib.next(){
        if (i % 2) == 0{
            count+=i;
        } else if i >= limit{
            break
        }
    }
    return count;
}

fn is_prime(input: u64)->bool{
    if input <= 3{
        return input > 1;
    }else if input % 2 == 0 || input % 3 == 0 {
        return false;
    }else{
        let mut i: u64 = 5;
        while i * i <= input{
            if input % i  == 0 || input % (i + 2) == 0{
                return false;
            }
            i += 6;
        }
    }
    return true;
}

pub fn problem_three(input: u64)->u64{
    let fl_input = input as f64;
    let sq_input = fl_input.sqrt();
    let inte_sq_input = sq_input as u64;
    for n in (1..inte_sq_input+1).rev(){
        if is_prime(n) && input % n == 0{
            return n;
        } 
    }
    return 1;
}

fn reverse(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c)
    }

    return result;
}

fn is_palindrome(input: u32)->bool{
    let s: String = input.to_string();
    let sr: String = reverse(&*s);
    match s == sr{
        true => true,
        _ => false
    }
}

pub fn problem_four()->u32{
    let mut temp: u32 = 0;
    for n in 100..999{
        for a in 100..999{
            if is_palindrome(a*n){
                if a*n > temp{
                    temp = a*n
                }
            }
        }
    }
    return temp;
}

fn gcd(mut a: u32, mut b: u32)->u32{
    while (a % b) != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    return b;
}

pub fn problem_five(range: u32)->u32{
    let mut ans: u32 = 1;
    for i in 1..range+1{
        ans *= i / gcd(i,ans);
    }
    return ans
}

pub fn problem_six(range: u32)->u32{
    let vec: Vec<u32> = (1..range+1).collect();
    let sum: u32 = vec.iter().sum();
    let double_sum: u32 = sum.pow(2);
    let vec2: Vec<_> = vec.iter().map(|x| x.pow(2)).collect();
    let sq_sum: u32 = vec2.iter().sum();
    return double_sum - sq_sum;
}

pub fn problem_seven(nth_prime: u32)->u64{
    let mut count: u32 = 0;
    let mut nums: u64 = 1;
    loop{
        if is_prime(nums){
            count += 1;
            if count == nth_prime{
                break;
            }
        }
        nums += 1;
    }
    return nums;
}

fn calc_slice(input: &str)->u128{
    let numbers = input.chars().map(|c| c.to_digit(10));
    let mut answer: u128 = 1;
    for num in numbers{
        match num {
            Some(num) => answer *= num as u128,
            None => println!("Error parsing input"),
        }
    }
    return answer;
}

pub fn problem_eight(input: String, sub: usize)->u128{
    let total_length: usize = input.len();
    let mut largest: u128 = 0;
    let mut largest_slice: &str = "";
    for i in 0..total_length-sub{
        let slice = &input[i..(i+sub)];
        let tmp = calc_slice(slice);
        if tmp > largest{
            largest = tmp;
            largest_slice = slice;
        }
    }
    println!("The largest product is {} from {}", largest, largest_slice);
    return largest;
}

pub fn problem_nine(triplet_sum: i32)-> i32{
    for a in 1..triplet_sum {
        for b in 1..triplet_sum{
            let c = triplet_sum - a - b;
            if (a*a + b*b) == c*c{
                return a*b*c;
            }
        }
    }
    return 1;
}

pub fn problem_ten(sum_total: u128)-> u128{
    let mut sum : u128 = 0;
    for a in 1..sum_total{
        
        if is_prime(a as u64){
            sum += a;
        }

    }
    return sum;
}


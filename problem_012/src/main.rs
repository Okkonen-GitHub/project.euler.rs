
fn main() {
    /* This was just to test the algorithms 
    for i in 1..11 {
        let result = triange_numbers(i);
        println!("{}: {}", i, result);
        println!("Divisors {} for {}", number_of_divisors(result), result);
    }
    */
    let mut i = 2;
    let result = loop {
        if number_of_divisors(triange_numbers(i)) >= 300 {
            break i;
        }
        i += 1;
        // println!("{}. Triangle number: {}, has {} divisors", i, triange_numbers(i), number_of_divisors(triange_numbers(i)));
    };
    println!("{}. Triangle number: {}, has {} divisors", i, triange_numbers(result), number_of_divisors(triange_numbers(result)));
}


fn triange_numbers(n: usize) -> u64 {
    // inefficient algorithm
    // for i in 2..n+1 {
    //     result += i as u64;
    // } 
    // 1+2+...+n = n * (n + 1) / 2 
    let result: u64 = n as u64 * (n as u64 + 1) / 2;
    result
}
/* this is also not a good algorithm
fn number_of_divisors(n: u64) -> u32 {
    let mut divisors: u32 = 1;
    if n == 1 {
        return 1u32;
    }
    for i in 2..(n/2)+1 {
        if n % i == 0 {
            divisors += 1;
        }
    }
    divisors += 1;
    divisors
} */

// This is *somehow* really efficient, don't know why
fn number_of_divisors(n: u64) -> u64 {
    let end = (n as f64).sqrt();
    let mut result = 2;
    for i in 1..(end as u64)+1 {
        if n % i == 0 {
            result += 2;
        }
    }
    if end * end == n as f64 {
        result -= 1;
    }
    result
}

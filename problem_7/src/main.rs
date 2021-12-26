fn main() {
    let result = gen_prime();
    println!("10001st prime: {}", result);
}


fn gen_prime() -> u64 { // Just brute force it by counting every prime until we find the 10001st prime
    let mut counter = 1;    
    let mut i:u64 = 3; // first prime is 2 so we start the counter from 1 and start checking from 3
    let re = loop { // this loop probably could be a for loop
        if is_prime(i) {
            counter += 1;
            if counter == 10001 {
                break i;
            }
        }
        i += 2; // increment by 2 because even numbers aren't primes
    };
    re
}

fn is_prime(n: u64) -> bool {
    for i in 2..(n/2)+1 { // only check divisibility up till 1+n/2 to reduce checking (and runtime) by half
        if n % i == 0 {
            return false
        }
    }
    true
}
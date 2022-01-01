// Didn't want to use lists for this one. So just adding each prime to the sum as they are discovered


fn main() {
    let mut sum: u64 = 0;
    // generate primes
    for i in 2..2_000_001 {
        if is_prime(&i) {
            sum += i;
        }
    }
    println!("prime: {}", sum);
    // println!("{}", sum);
}


fn is_prime(n: &u64) -> bool {
    for i in 2..((*n as f64).sqrt() as u64 +1) { // need to dereference n (with *n) as it's a reference
        if n % i == 0 {
            return false;
        }
    }
    true
}

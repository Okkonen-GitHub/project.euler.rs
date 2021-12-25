fn main() {
    // let primes = larges_prime_factor(131951214465);
    let primes = larges_prime_factor(600851475143);
    println!("Largest prime factor: {:?}", primes);
    println!("Prime? {}", is_prime(&primes));
    
    let primes = larges_prime_factor(131951214465);
    println!("Largest prime factor: {:?}", primes);
    
    let primes = larges_prime_factor(13195121446);
    println!("Largest prime factor: {:?}", primes);
    
    let test =vec![2, 13,507504671];
    println!("{:?} Prime? {:?}", &test, is_prime(&test));

    println!("{:?}", larges_prime_factor(94606939379));
}


fn larges_prime_factor(mut n: u64) -> Vec<u64> { // algorithm from https://helloacm.com/efficient-prime-factorization-algorithm-integer-factorization/
    let mut factors: Vec<u64> = Vec::new();
    while n % 2 == 0 { // as long as you can divide by 2, do so
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i*i <= n {
        
        while n % i == 0 {
            n /= i;
            factors.push(i);
        }
        i += 2;
    }
    if n > 2 { // If after trying numbers up until sqrt(n) we still haven't found stuff, n is prime
        factors.push(n);
    }
    factors
    
}

fn is_prime(n: &Vec<u64>) -> bool { // check entire vector of primes at a time for efficiency
    // println!("{}", n);
    for p in n.iter() {
        for i in 2..((p+1)/2) {
            // println!("ayo{}", i);
            if p % i == 0 {
                return false;
            } 
        }
        return true
    }
    true
}
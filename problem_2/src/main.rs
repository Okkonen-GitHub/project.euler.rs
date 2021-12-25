

fn main() {
    
    let r = fibonacci();
    let mut total = 0; // calc the sum of every element in the Vector r (list)
    for item in r.iter() {
        total += item;
    }
    println!("{}", total);
}

fn fibonacci() -> Vec<u32> {
    let mut even_nums: Vec<u32> = vec![2]; // initialize a vector with the first fibonacci numbers (not including 1 because it's odd)
    const BIGGEST_N: u32 = 4_000_000;
    
    // calculate the fibonacci numbers iteratively
    let mut a = 1;
    let mut b = 2;
    loop {
        let swap = b;
        b = a + b;
        if b > BIGGEST_N { // if result is over 4M, stop
            break;
        }
        a = swap;
        if b % 2 == 0 {
            even_nums.push(b); // if result is even, add it to the list (vector) 
        }
    }
    even_nums // return
}

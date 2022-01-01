fn main() {
    let answer = square_of_sum(100) - sum_of_squares(100); 
    println!("{}", answer);
}


fn sum_of_squares(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..n+1 { // (1^2 + 2^2 + 3^2 + 4^2 + ... + n^2)
        sum += i * i
    }
    sum
}


fn square_of_sum(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..n+1 { // (1 + 2 + 3 + 4 + ... + n)^2
        sum += i;
    }
    sum * sum
}
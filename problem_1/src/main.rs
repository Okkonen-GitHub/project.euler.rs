fn main() {
    let n = 1000;
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            println!("{}", i);
            sum += i
        }
    }
    println!("Sum: {}", sum);
}

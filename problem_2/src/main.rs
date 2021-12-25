

fn main() {
    
    let r = fibonacci();
    let mut total = 0;
    for item in r.iter() {
        total += item;
    }
    println!("{}", total);
}

fn fibonacci() -> Vec<u32> {
    let mut even_nums: Vec<u32> = vec![2];
    const BIGGEST_N: u32 = 4_000_000;
    let mut a = 1;
    let mut b = 2;
    // let i = 0;
    loop {
        let swap = b;
        b = a + b;
        if b > BIGGEST_N {
            break;
        }
        a = swap;
        if b % 2 == 0 {
            even_nums.push(b);
        }
    }
    even_nums
}

fn main() {
    println!("Palindrome num: {}", gen());
}

fn gen() -> u32 {
    const MAX: u32 = 999;
    
    let mut largest = 1;

    for i in 1..MAX { // just try every possible number and check them
        for j in 1..MAX {
            if j*i == reverse(j*i) {
                if largest < j*i {
                    largest = j*i;
                    // println!("{}*{} = {}", j, i, j*i);
                }
            }
        }
    }
    largest
}

fn reverse(n: u32) -> u32 {
    let mut result: String = String::new();
    let m = n.to_string(); // convert the number to a string
    for p in m.chars() { // then reverse the string character by character 
        result.insert(0, p);
    }
    let f: u32 = match result.parse() { // convert the string back to a number with .parse()
        Ok(n) => n,
        Err(_) => 1
    };
    f
}

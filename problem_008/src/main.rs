fn main() {
    // treat the numbers as a long string slice (&str)
    let nums: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let largest4 = largest_product(&nums, 4); // test if the algorithm works
    println!("{}", largest4);
    let largest13 = largest_product(&nums, 13);
    println!("{}", largest13);
}

fn largest_product(nums: &str, n: usize) -> i64 { // i32 is not big enough (could use u32 i guess)
    let mut largest: i64 = 0;
    for i in 0..(nums.chars().count()-n+1) { // i: 0 to (1000-13+1) (20 rows of 50 nums = 10000 digits), where -n prevents from trying to calculate digits 1001-1014
        let result = digit_product(&nums[i..i+n]); // then we calculate the product of every digit in a 13 digit slice of the string (0-13), (1-14), (2-15) ... (988-1000)
        if result > largest {
            largest = result;
            // println!("{}", largest);
        }
    }
    largest
}

fn digit_product(s: &str) -> i64 {
    let mut result: i64 = 1;
    for c in s.chars() { // loop through the chars of a &str
        let a: i64 = match c.to_digit(10) { // convert char to a i64, using a match block to handle errors
            Some(num) => num as i64,
            None => 1,
        };
        result *= a;
    }
    result
}
fn main() {
    for a in 1..499 { // because the outer loop is a, a will be smaller than b most likely
        for b in 1..499 {
            let c = 1000 - a -b; // calculcate c instead of having to check or generate it
            if pythagoras(&a, &b, &c) {
                println!("{}", a*b*c);
                return;
            }
        }
    }
}


// fn comparisons(a: &i32, b: &i32, c: &i32) -> bool {
//     if a < b && b < c {
//         return true;
//     }
//     false
// }

fn pythagoras(a: &i32, b: &i32, c: &i32) -> bool {
    if (a*a) + (b*b) == (c*c) {
        return true;
    }
    false
}

// fn sum(a: &i32, b: &i32, c: &i32) -> bool {
//     if a+b+c == 1000 {
//         return true;
//     }
//     false
// }
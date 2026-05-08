fn main() {
    let a = [1, 3, 7, 4, 9];
    let b = [1, 3, 7, 9];

    match find_first_even(&a) {
        Some(val) => println!("first case output: {}", val),
        None => println!("first case output: No even found"),
    }
    if let Some(val) = find_first_even(&b) {
        println!("first case output: {}", val);
    } else {
        println!("first case output: No even found");
    }
    // match find_first_even(&b) {
    //     Some(val) => println!("first case output: {}", val),
    //     None => println!("first case output: No even found"),
    // }
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for x in numbers {
        if x % 2 == 0 {
            return Some(*x);
        }
    }

    None
}

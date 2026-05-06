fn main() {
    let x: String = String::from("shivam");
    let y: String = String::from("long-shivam");
    println!("{}, {}", x, y);

    let longer = temp_func(&x, &y);

    println!("longer: {}", longer);
}

fn temp_func<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

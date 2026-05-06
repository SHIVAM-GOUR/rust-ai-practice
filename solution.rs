fn main() {
    let mut x = String::from("shivam");
    println!("{}", x);

    temp_func(&mut x);
    println!("{}", x);
}

fn temp_func(x: &mut String) {
    *x = String::from("changed value");
}
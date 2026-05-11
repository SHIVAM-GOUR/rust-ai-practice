fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("{:?}", v);

    for x in &v {
        println!("{}", x);
    }

    println!("again");
    for x in v.iter() {
        println!("{}", x);
    }

    println!("{:?}", v);
}

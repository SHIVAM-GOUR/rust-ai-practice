#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
    Northeast,
}

fn main() {
    let d = Direction::South;
    let x = describe(&d);
    println!("{}", x);
    let p = describe(&d);
    println!("{}", p);

    // let n = Direction::North;
    // let y = describe(n);
    // println!("{}", y);
}

fn describe(d: &Direction) -> &'static str {
    match d {
        Direction::East => "Going East",
        Direction::West => "Going West",
        Direction::North => "Going North",
        Direction::South => "Going South",
    }
}

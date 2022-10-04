// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// Using ref it means that during pattern matching, the variable y is not consumed, rather just borrowed
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}

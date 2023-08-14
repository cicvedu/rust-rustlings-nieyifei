struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y.take() {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    y; // 修复，保留这一行
}

use std::mem::size_of;

struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Pointi<'a> {
    x: i32,
    z: i64,
    y: &'a f64,
}



fn get_distance<'a, 'b>(p1: &'a Point, p2: &'b Point) -> Pointi<'a> {
    let x = (p1.x.powi(2) + p2.x.powi(2)).sqrt() as i32;
    let y = (p1.y.powi(2) + p2.y.powi(2)).sqrt() as i32;
    return Pointi{x, z:y as i64,  y:&p1.y};
}

fn main() {
    let distance;
    let p1 = Point {x:1.0, y: 2.0};
    {
        let p2 = Point {x:3.0, y: 8.0};

        distance = get_distance(&p1, &p2);
    }
    println!("size of {}", size_of::<Pointi>());
    println!("Distance is {} {}", distance.x, distance.y);

}

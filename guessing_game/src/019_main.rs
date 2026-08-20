struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
impl Point<f32> {
    fn distance_from_orign(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let p = Point {
        x: 3.0,
        y: 4.3,
    };
    let d = p.distance_from_orign();
    println!("distance = {}", d);
}

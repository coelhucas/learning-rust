mod math;

pub use crate::math::vector;

fn main() {
    let vec = vector::Vector2 { x: 1.0, y: 2.0 };
    let unit_vec = vec.normalized();
    println!("{:#?}", unit_vec);
}

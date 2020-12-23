#[derive(Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalized(&self) -> Vector2 {
        let v = Vector2 {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
        };

        v
    }
}

fn main() {
    let point_a = Vector2 { x: 8.0, y: 12.0 };
    let unit_vec = point_a.normalized();
    println!("Magnitude of a is ~{}", point_a.magnitude());
    println!("{:#?} normalized is {:#?}", point_a, unit_vec);

    // Proving that the result is right:
    println!("{}", unit_vec.magnitude());
}

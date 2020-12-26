#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vector2 {
        let v = Vector2 {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
        };

        v
    }
}

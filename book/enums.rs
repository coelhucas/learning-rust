enum Color {
    Red,
    Green,
    Blue,
}

struct Shape {
    color: Color,
    name: String,
}

// in rust I can also do
#[derive(Debug)]
enum Colour {
    Hex(String),
    RGBA(u8, u8, u8, u8),
}

fn main() {
    let blue = Color::Blue;
    let blue_circle = Shape {
        color: blue,
        name: String::from("circle"),
    };

    let white = Colour::Hex(String::from("#FFF"));
    let black = Colour::RGBA(0, 0, 0, 1);
    println!("Name of the color is {}", color_string(blue_circle.color));
    println!("{:#?} is a {}", black, color_type(&black));
    println!("{:#?} is a {}", white, color_type(&white));
}

fn color_string(color: Color) -> String {
    match color {
        Color::Red => String::from("red"),
        Color::Green => String::from("green"),
        Color::Blue => String::from("blue"),
    }
}

fn color_type(color: &Colour) -> String {
    match color {
        Colour::Hex(_hex) => String::from("hex"),
        Colour::RGBA(_r, _g, _b, a) => {
            println!("Alpha value is: {}", a);
            String::from("rgba")
        }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    let bottom_right = Point { x: 5.2, y: 0.2};

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("===========");
    println!("_rectangle.top_left.x = {}", _rectangle.top_left.x);
    println!("_rectangle.top_left.y = {}", _rectangle.top_left.y);
    println!("_rectangle.bottom_right.x = {}", _rectangle.bottom_right.x);
    println!("_rectangle.bottom_right.y = {}", _rectangle.bottom_right.y);
    println!("===========");

    println!("aread of rectangle = {}", rect_area(_rectangle));
}

fn rect_area(rect: Rectangle) -> f32 {
    let top_left: Point = rect.top_left;
    let bottom_right = rect.bottom_right;

    (top_left.x - bottom_right.x) * (top_left.y - bottom_right.y)
}

fn sq (point: Point, side: f32) -> Rectangle {
    let mut rect = Rectanlge {
        top_left = Point {
            x: left_edge,
            y: top_edge
        },
        bottom_right: bottom_right,
    };
    todo!()
}

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

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

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity
    // 1)
    let actual_rectangle = Rectangle {
        top_left: Point { x: 2.5, y: 5.11 },
        bottom_right: Point { x: 10.6, y: 1.04 }
    };
    rect_area(&actual_rectangle);

    // 2)
    let point4square: Point = Point { x: 5.0, y: 5.0 };
    let size4square: f32 = 10.0;

    let mynewsquare: Rectangle = square(&point4square, size4square);
    println!("My new square is: Top Left: {:?}, {:?}. Bottom Right: {:?}, {:?}", 
        mynewsquare.top_left.x, mynewsquare.top_left.y, mynewsquare.bottom_right.x, mynewsquare.bottom_right.y);
}


// Activity:
//  Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
//  Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.

fn rect_area(rect: &Rectangle) {
    // Get edges
    let Point { x: rect_left_edge, y: rect_top_edge } = rect.top_left;
    let Point { x: rect_right_edge, y: rect_bottom_edge } = rect.bottom_right;

    // Calculate differences
    let height: f32 = rect_top_edge - rect_bottom_edge;
    let length: f32 = rect_left_edge - rect_right_edge;

    // Calculate area and print
    let rect_area: f32 = height * length;
    // To display a positive number, first thing that came to mind.
    let pos_area = f32::powf(rect_area, 2.0).sqrt();

    // Didn't work as I expected, but leaving here for my reference
    let another_pos_area: u32 = rect_area as u32;

    println!("Area of rectangle is: {:?}", pos_area);
    println!("Area of rectangle in integer is: {:?}", another_pos_area);
}

fn square(pnt: &Point, size: f32) -> Rectangle{
    return Rectangle { 
        top_left: Point { x: pnt.x, y: pnt.y }, 
        bottom_right: Point { x: pnt.x + size, y: pnt.y - size } 
    };
}
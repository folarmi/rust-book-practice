// fn main() {
//     struct User {
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//         // extra:"jksdfjkdsnj",
//     };

//    let _user2 = User {
//     email: String::from("another@example.com"),
//     ..user1
//    };

// //    Tuple Structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// let _black = Color(0, 0, 0);
// let _origin = Point(0, 0, 0);

// }

fn main() {
    let scale = 2;
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: dbg!(10 * scale),
        height:40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(3);


    // used to debug, passing a reference because we don't want the debug fn to take ownership of rect2
    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle using tuples is {} square pixels.",
        area_with_tuples(rect1)
    );

    // println!("rect1 is {rect2:?}");
    println!(
        "The area of the rectangle using structs is {} square pixels.",
        area_with_structs(&rect2)
    );
    println!("The area of the rectangle using method is {}", rect2.area_impl());
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));
    println!("Square {sq:?}");

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width)
    }
}

// Calculating the area of a rectangle without structs
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Calculating the area of a rectangle with tuples
fn area_with_tuples (dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Calculating the area of a rectangle with structs
fn area_with_structs (rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// The first parameter of a method(function in a struct) is always self
// self represents the instance of the struct the method is being called on 

// If we wanted to change the instance that we’ve called the method 
// on as part of what the method does, we’d use &mut self as the first parameter
impl Rectangle {
    fn area_impl (&self) -> u32 {
        self.width * self.height
    }

// A method can be name after one of the structs fields
    fn width (&self) -> bool {
        self.width > 0
    }

    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions that don’t have self as their first parameter
    fn square (size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}



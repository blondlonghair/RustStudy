// use std::ops::Add;

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type OutPut = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
// }



// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }

// fn main() {
//     let person = Human;
//     Pilot::fly(&person);
//     Wizard::fly(&person);
//     person.fly();
// }



// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from("Spot")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }

// fn main() {
//     println!("A baby dog is calling a {}", <Dog as Animal>::baby_name());
// }



// use std::fmt;

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", "*".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl OutlinePrint for Point {}

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut::Formater) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }



use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formater) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
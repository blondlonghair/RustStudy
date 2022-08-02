use std::ops::Deref;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};



// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }



// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("CustomSmartPointer의 데이터 '{}'를 해제합니다.", self.data);
//     }
// }



// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }



// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match *self {
//             Cons(_, ref item) => Some(item),
//             Nil => None,
//         }
//     }
// }



#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // let b = Box::new(5);
    // println!("{}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));



    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);



    // let c = CustomSmartPointer {data: String::from("내 데이터")};
    // println!("CustomSmartPointer를 생성했습니다.");
    // drop(c);
    // println!("CustomSmartPointer를 main 함수의 긑에 도달하기 전에 해제합니다.");



    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("a를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("b를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("c를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    // }
    // println!("c가 범위를 벗어난 후의 카운터 = {}", Rc::strong_count(&a));



    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a 수정 후 = {:?}", a);
    // println!("b 수정 후 = {:?}", b);
    // println!("c 수정 후 = {:?}", c);



    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a 최초 rc 카운트 = {}", Rc::strong_count(&a));
    // println!("a의 다음 아이템 = {:?}", a.tail());

    // let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    // println!("b를 생성한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
    // println!("b의 최초 rc 카운트 = {}", Rc::strong_count(&b));
    // println!("b의 다음 아이템 = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("a를 변경한 후 b의 rc 카운트 = {}", Rc::strong_count(&b));
    // println!("a를 변경한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));



    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, Weak = {},",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, Weak = {},",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        println!(
            "leaf strong = {}, Weak = {},",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, Weak = {},",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

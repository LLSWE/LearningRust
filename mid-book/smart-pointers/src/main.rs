//COns List ds
// enum List {
//     Cons(i32, List),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
//     let x = Box::new(5);
// }
//
//

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     let x = Box::new(5);
// }
//
//
// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
//
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }
//
// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });
//
//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
//
//     {
//         let branch = Rc::new(Node {
//             value: 5,
//             parent: RefCell::new(Weak::new()),
//             children: RefCell::new(vec![Rc::clone(&leaf)]),
//         });
//
//         *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
//
//         println!(
//             "branch strong = {}, weak = {}",
//             Rc::strong_count(&branch),
//             Rc::weak_count(&branch),
//         );
//
//         println!(
//             "leaf strong = {}, weak = {}",
//             Rc::strong_count(&leaf),
//             Rc::weak_count(&leaf),
//         );
//     }
//
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
// }

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota! ");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up overz 75% of your quota! ");
        }
    }
}

fn main() {}

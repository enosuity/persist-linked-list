#![allow(unused)]

#[cfg(feature = "plist")]

use std::{cell::RefCell, clone, rc::Rc, thread};

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    previous: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct Series<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Series<T> {
    pub fn new() -> Self {
        Series { 
            head:  None,
        }
    }

    pub fn prepend(&self, value: T) -> Series<T> {
        let new_node = Node {
            value,
            previous: self.head.clone(),
        };

        Series {
            head: Some(Rc::new(RefCell::new(new_node))),
        }
    }

    pub fn iter(&self) -> SeriesIterator<T> {
        SeriesIterator {
            current: self.head.clone()
        }

    }
}

pub struct SeriesIterator<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for SeriesIterator<T> 
where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
      self.current.clone().map(|node| {
        let node_borrow =  node.borrow();
        self.current = node_borrow.previous.clone();
        node_borrow.value.clone()        
      })
    }
    
}

#[derive(Debug)]
pub enum Roll {
    Big(Rc<RefCell<i32>>, Rc<Roll>),
    Nothing,
}

pub use Roll::{Big, Nothing};

fn with_ref() {
    let a = [0; 10*1025];
    
    println!("\na@{:p}", &a);

    let handle = thread::spawn(move || {
        println!("\na@{:p}", &a);
    });

    handle.join();
}

pub fn add_bang(word: Rc<RefCell<String>>) {
    let mut s = word.borrow_mut();
    // dbg!(Rc::strong_count(&word));
    s.push('!'); 
}


// ==================== main.rs =================
// use std::{cell::RefCell, rc::Rc};
// use persist_link::add_bang;
// use persist_link::{Big, Nothing, Node, Series};



// fn main() {
//     println!("=========== Refcell with Ref ==========\n");

//     let greeting_text = String::from("Hello, world");
    

//     let greet = Rc::new(RefCell::new(greeting_text));
//     // dbg!(Rc::strong_count(&greet));
//     add_bang(Rc::clone(&greet));
    
//     // dbg!(Rc::strong_count(&greet));
    

//     println!("\n{:?}", greet);
    
//     // ========================================================

//     let val = Rc::new(RefCell::new(15));

//     let a = Rc::new(Big(Rc::clone(&val), Rc::new(Nothing)));

//     let b = Big(Rc::new(RefCell::new(50)), Rc::clone(&a));
//     let c = Big(Rc::new(RefCell::new(80)), Rc::clone(&a));

//     // println!("val: {:?}", val);
//     println!("a : {:?}", a);

//     *val.borrow_mut() += 25;

    
//     println!("\n\na : {:?}", a);
//     println!("b : {:?}", b);
//     println!("c: {:?}", c);

//     // ======================== Persistant Link List ===========================
//     let list = Series::new();
//     let list = list.prepend(10);
//     let list = list.prepend(20);
//     let list = list.prepend(30);
//     let list = list.prepend(40);

//     println!("\n\nPersist List:\n{:?}\n\n", list);

//     for item in list.iter() {
//         println!("item: {}", item);
//     }
// }

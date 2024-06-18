#![allow(unused)]

use std::{cell::RefCell, clone, rc::Rc};

#[derive(Debug)]
struct Node<T> {
    value: T,
    previous: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct Series<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Series<T> {
    fn new() -> Self {
        Series { 
            head:  None,
        }
    }

    fn prepend(&self, value: T) -> Series<T> {
        let new_node = Node {
            value,
            previous: self.head.clone(),
        };

        Series {
            head: Some(Rc::new(RefCell::new(new_node))),
        }
    }

    fn iter(&self) -> SeriesIterator<T> {
        SeriesIterator {
            current: self.head.clone()
        }

    }
}

struct SeriesIterator<T> {
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


fn main() {
    println!("Hello, world!");

    let list = Series::new();
    let list = list.prepend(10);
    let list = list.prepend(20);
    let list = list.prepend(30);
    let list = list.prepend(40);
    println!("list: {:?}", list);

    for item in list.iter() {
        println!("item: {}", item);
    }
}

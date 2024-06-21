// #![allow(unused)]

use std::{cell::RefCell, fmt::Display, rc::Rc};

#[cfg(feature = "Btree")]

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<Node<T>>>>,
    pub right: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T> {
    pub root: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Tree<T>
where 
    T: PartialOrd + Clone + Display
{
    pub fn new(val: T) -> Self {
        let node = Rc::new(RefCell::new(Node {
            value: val,
            left: None,
            right: None,
        }));

        Tree {
            root: Some(Rc::clone(&node))
        }
    }

    pub fn print(&self) {
        if let Some(ref node) = self.root {
            Tree::tree_view(node, 0);
        }               
    }

    fn tree_view(node: &Rc<RefCell<Node<T>>>, depth: usize) {
        let node_borrow = node.borrow();

        if let Some(ref right) = node_borrow.right {
            Tree::tree_view(right, depth + 1);
        }

        println!("{:space$}{}", " ", node_borrow.value, space = depth * 5);
        
        if let Some(ref left) = node_borrow.left {
            Tree::tree_view(left, depth + 1);
        }
    }

    pub fn insert(&self, item: T) {
       if let Some(ref root) = self.root {
        Self::insert_node(Rc::clone(root), item);
       }        
    }

    fn insert_node(node: Rc<RefCell<Node<T>>>, item: T) {        
        let mut borrow_node = node.borrow_mut();
        if borrow_node.value > item {
            match borrow_node.left {
                Some(ref t) => Tree::insert_node(Rc::clone(t), item),
                None => {
                    let new_node = Rc::new(RefCell::new(
                        Node {
                            value: item,
                            left: None,
                            right: None,
                        }
                    ));
                    borrow_node.left = Some(new_node);
                }
            }
        } 
        else if borrow_node.value < item {
            match borrow_node.right {
                Some(ref t) => Tree::insert_node(Rc::clone(t), item),
                None => {
                    let new_node = Rc::new(RefCell::new(
                        Node {
                            value: item,
                            left: None,
                            right: None,
                        }
                    ));
                    borrow_node.right = Some(new_node);
                }
            }  
        }
        
    }
}

pub fn run() {
    let tree = Tree::new(12);
    tree.insert(5);
    tree.insert(27);
    tree.insert(17);
    tree.insert(3);
    tree.insert(11);
    tree.insert(20);
    tree.insert(45);
    tree.insert(16);
    tree.print();
}


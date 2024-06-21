#![allow(unused)]

use std::{cell::RefCell, io::{self, BufRead}, rc::Rc};
use simple_binary_tree::run;


fn main() {
    
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    println!("enter numbers separated by space i.e 10 5 15 3 7 12 18 : ");
    let list: Vec<i32> = lines.next().unwrap().unwrap()
                              .trim()
                              .split(" ")
                              .map(|num| num.parse::<i32>().unwrap() )
                              .collect();
            
         

    println!("\nBinary Tree View: \n");
        
    run(&list)    
}

#[macro_use]
mod myvec;
mod linkedlist;
mod deque;
mod stack;
mod utils;

#[allow(unused)]
use linkedlist::LinkedList;
#[allow(unused)]
use myvec::MyVec;
#[allow(unused)]
use deque::SimpleDeque;
#[allow(unused)]
use stack::Stack;

use crate::utils::get_input;



fn main() {
    let mut deque: SimpleDeque<usize> = SimpleDeque::new(10);
    for i in 0..10  {
        deque.push(i).unwrap(); 
    }
    println!("{}", deque);

    let mut stack: Stack<usize> = Stack::new(10);
    for i in 0..10 {
        stack.push(i).unwrap();
    }
    println!("{}", stack);

    println!("{}", get_input::<String>("Enter a string: "));
}

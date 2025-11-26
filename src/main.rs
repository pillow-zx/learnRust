#[macro_use]
mod myvec;
mod linkedlist;
mod deque;
mod stack;

#[allow(unused)]
use linkedlist::LinkedList;
#[allow(unused)]
use myvec::MyVec;
#[allow(unused)]
use deque::SimpleDeque;
#[allow(unused)]
use stack::Stack;



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
}

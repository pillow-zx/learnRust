#[macro_use]
mod myvec;
mod linkedlist;

use linkedlist::LinkedList;
use myvec::MyVec;

fn main() {
    let mut vector: Box<MyVec<usize>> = Box::new(MyVec::new());

    for i in 0..15 {
        vector.push(i);
    }
    println!("{}", vector);

    let mut ls: LinkedList<usize> = LinkedList::new();

    for i in 0..15 {
        ls.push_back(i);
    }

    for val in ls.iter() {
        println!("{}", val);
    }
}

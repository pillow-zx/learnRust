use std::{mem::MaybeUninit};

#[allow(dead_code)]
#[derive(Debug)]
pub struct SimpleDeque<T> {
    data: Box<[MaybeUninit<T>]>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

#[allow(dead_code)]
impl<T> SimpleDeque<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Box::new_uninit_slice(capacity),
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        if self.size == self.capacity {
            return true;
        }
        false
    }

    pub fn push(&mut self, data: T) -> Result<(), &str>{
        if self.is_full() {
            return Err("Queue is fulled!");
        }
        self.data[self.tail].write(data);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    fn is_empty(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        false
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = unsafe {
            self.data[self.head].assume_init_read()
        };
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        Some(value)
    }
}

impl<T>  std::fmt::Display for SimpleDeque<T>
where 
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head;
        let mut count = 0;
        write!(f, "[")?;
        loop {
            if count == self.size {
                break;
            }
            let value = unsafe {
                self.data[current].assume_init_read()
            }; 
            if current == self.size - 1 {
                write!(f, "{}", value)?;
            } else {
                write!(f, "{}, ", value)?;
            }
            current = (current + 1) % self.capacity;
            count += 1;
        }
        write!(f, "]")
    }
}

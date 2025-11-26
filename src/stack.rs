use std::{mem::MaybeUninit};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Stack<T> {
    data: Box<[MaybeUninit<T>]>,
    top: usize,
    capacity: usize,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Box::new_uninit_slice(capacity),
            top: 0,
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        if self.top == self.capacity {
            return true;
        }
        false
    }

    pub fn push(&mut self, data: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Stack is fulled");
        }
        self.data[self.top].write(data);
        self.top += 1;
        Ok(())
    }

    fn is_empty(&self) -> bool {
        if self.top == 0 {
            return true;
        }
        false
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = unsafe { self.data[self.top].assume_init_read() };
        self.top -= 1;
        Some(value)
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

impl<T> std::fmt::Display for Stack<T>
where 
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = 0;        
        write!(f, "[")?;
        loop {
            if current == self.top {
                break;
            }
            let value = unsafe {
                self.data[current].assume_init_read()
            };
            if current == self.top - 1 {
                write!(f, "{}", value)?;
            } else {
                write!(f, "{}, ", value)?;
            }
            current += 1;
        }
        write!(f, "]")
    } 
}

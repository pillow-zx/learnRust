use std::{
    alloc::{Layout, alloc, dealloc, realloc},
    ptr,
};

const DEFAULTSIZE: usize = 10;

#[derive(Debug)]
pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

#[allow(dead_code)]
impl<T> MyVec<T> {
    pub fn new() -> Self {
        let ptr = unsafe { alloc(Layout::array::<T>(DEFAULTSIZE).unwrap()) as *mut T };
        if ptr.is_null() {
            dbg!("[MyVec] Memory alloc failed");
        }
        Self {
            ptr,
            len: 0,
            cap: DEFAULTSIZE,
        }
    }

    #[inline]
    fn layout(cap: usize) -> Layout {
        if cap == 0 {
            Layout::new::<T>()
        } else {
            Layout::array::<T>(cap).unwrap()
        }
    }

    fn grow(&mut self, new_cap: usize) {
        let new_layout = Self::layout(new_cap);

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Self::layout(self.cap);
            unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) }
        } as *mut T;

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow(self.cap * 2);
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(ptr::read(self.ptr.add(self.len))) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.ptr.add(index)) }
        } else {
            None
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }
}

impl<T> std::fmt::Display for MyVec<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.len);
            write!(f, "[")?;

            for (i, item) in slice.iter().enumerate() {
                write!(f, "{}", item)?;

                if i + 1 != self.len {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                for i in 0..self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                }
                let layout = Self::layout(self.cap);
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

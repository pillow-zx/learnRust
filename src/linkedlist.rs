#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(head) if head.next.is_none() => self.head.take().map(|node| node.data),
            Some(_) => {
                let mut current = self.head.as_mut().unwrap();
                while let Some(ref mut next_node) = current.next {
                    if next_node.next.is_none() {
                        return current.next.take().map(|node| node.data);
                    }
                    current = current.next.as_mut().unwrap();
                }
                None
            }
        }
    }

    pub fn iter<'a>(&'a self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}

impl<T> std::fmt::Display for Node<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        while let Some(node) = self.next.as_ref() {
            write!(f, "{}", node)?;
        }
        write!(f, "{} ]", self.data)
    }
}

impl<T> std::fmt::Display for LinkedList<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.as_ref();
        write!(f, "[")?;
        while let Some(node) = current {
            write!(f, "{}", node.data)?;
            current = node.next.as_ref();
            if current.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = node.next.as_deref();
                Some(&node.data)
            }
            None => None,
        }
    }
}

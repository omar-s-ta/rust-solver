type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn with_elem(elem: T) -> Self {
        let node = Node { elem, next: None };
        let mut new_node = Box::new(node);
        let raw_node: *mut Node<T> = &mut *new_node;

        Self {
            head: Some(new_node),
            tail: raw_node,
            len: 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, elem: T) {
        let node = Node { elem, next: None };
        let mut new_node = Box::new(node);
        let raw_node: *mut Node<T> = &mut *new_node;

        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
        }
        self.tail = raw_node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            self.len -= 1;
            node.elem
        })
    }

    pub fn append(&mut self, other: &mut Self) {
        if other.is_empty() {
            return;
        }
        if self.head.is_none() {
            self.head = other.head.take();
        } else {
            unsafe {
                (*self.tail).next = other.head.take();
            }
        }
        self.tail = other.tail;
        self.len += other.len;
        other.tail = std::ptr::null_mut();
        other.len = 0;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

use std::marker::PhantomData;

use crate::io::output::{Output, Writable};

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct Iter<'a, T> {
    next: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        unsafe {
            let node = &*self.next;
            self.next = node.next;
            Some(&node.elem)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cursor<T> {
    list: *mut List<T>,
    current: Link<T>,
}

impl<T> Cursor<T> {
    pub fn is_end(&self) -> bool {
        self.current.is_null()
    }

    pub fn dec(&mut self) {
        unsafe {
            if self.current.is_null() {
                self.current = (*self.list).tail;
            } else {
                self.current = (*self.current).prev;
            }
        }
    }

    pub fn inc(&mut self) {
        unsafe {
            if !self.current.is_null() {
                self.current = (*self.current).next;
            }
        }
    }
}

impl<T> PartialEq for Cursor<T> {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list && self.current == other.current
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn with_elem(elem: T) -> Self {
        let node = Box::new(Node {
            elem,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        });
        let raw_node = Box::into_raw(node);

        Self {
            head: raw_node,
            tail: raw_node,
            len: 1,
        }
    }

    pub fn clear(&mut self) {
        self.head = std::ptr::null_mut();
        self.tail = std::ptr::null_mut();
        self.len = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: std::ptr::null_mut(),
            prev: self.tail,
        });
        let raw_node = Box::into_raw(node);

        if self.tail.is_null() {
            self.head = raw_node;
        } else {
            unsafe {
                (*self.tail).next = raw_node;
            }
        }
        self.tail = raw_node;
        self.len += 1;
    }

    pub fn append(&mut self, other: &mut Self) {
        if other.is_empty() {
            return;
        }
        if self.head.is_null() {
            self.head = other.head;
        } else {
            unsafe {
                (*self.tail).next = other.head;
                (*other.head).prev = self.tail;
            }
        }
        self.tail = other.tail;
        self.len += other.len;
        other.clear();
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            let head = self.head;
            let next = (*self.head).next;
            self.head = next;
            if next.is_null() {
                self.tail = std::ptr::null_mut();
            } else {
                (*next).prev = std::ptr::null_mut();
            }
            self.len -= 1;
            let boxed = Box::from_raw(head);
            Some(boxed.elem)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            let tail = self.tail;
            let prev = (*self.tail).prev;
            self.tail = prev;
            if prev.is_null() {
                self.head = std::ptr::null_mut();
            } else {
                (*prev).next = std::ptr::null_mut();
            }
            self.len -= 1;
            let boxed = Box::from_raw(tail);
            Some(boxed.elem)
        }
    }

    pub fn begin(&mut self) -> Cursor<T> {
        Cursor {
            list: self,
            current: self.head,
        }
    }

    pub fn end(&mut self) -> Cursor<T> {
        Cursor {
            list: self,
            current: std::ptr::null_mut(),
        }
    }

    pub fn insert(&mut self, cursor: &Cursor<T>, elem: T) -> Cursor<T> {
        debug_assert_eq!(self as *mut List<T>, cursor.list);

        let node = Box::new(Node {
            elem,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        });
        let raw_node = Box::into_raw(node);

        unsafe {
            if cursor.current.is_null() {
                (*raw_node).prev = self.tail;
                if self.tail.is_null() {
                    self.head = raw_node;
                } else {
                    (*self.tail).next = raw_node;
                }
                self.tail = raw_node;
            } else {
                let next = cursor.current;
                let prev = (*next).prev;
                (*raw_node).next = next;
                (*raw_node).prev = prev;
                (*next).prev = raw_node;
                if prev.is_null() {
                    self.head = raw_node;
                } else {
                    (*prev).next = raw_node;
                }
            }
        }

        self.len += 1;
        Cursor {
            list: self,
            current: raw_node,
        }
    }

    pub fn erase(&mut self, cursor: &mut Cursor<T>) -> Cursor<T> {
        debug_assert_eq!(self as *mut List<T>, cursor.list);

        if cursor.current.is_null() {
            return Cursor {
                list: self,
                current: std::ptr::null_mut(),
            };
        }

        unsafe {
            let node = cursor.current;
            let next = (*node).next;
            let prev = (*node).prev;

            if prev.is_null() {
                self.head = next;
            } else {
                (*prev).next = next;
            }
            if next.is_null() {
                self.tail = prev;
            } else {
                (*next).prev = prev;
            }

            self.len -= 1;
            cursor.current = next;
            let boxed = Box::from_raw(node);
            drop(boxed);

            Cursor {
                list: self,
                current: next,
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while !self.head.is_null() {
            unsafe {
                let node = self.head;
                self.head = (*node).next;
                let boxed = Box::from_raw(node);
                drop(boxed);
            }
        }
        self.tail = std::ptr::null_mut();
        self.len = 0;
    }
}

impl<T: Writable> Writable for List<T> {
    fn write(&self, output: &mut Output) {
        for e in self.iter() {
            e.write(output);
        }
    }
}


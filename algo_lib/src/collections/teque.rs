use std::{collections::VecDeque, ops::Index};

pub struct Teque<T> {
    a: VecDeque<T>,
    b: VecDeque<T>,
}

impl<T> Teque<T> {
    pub fn new() -> Self {
        Self {
            a: VecDeque::new(),
            b: VecDeque::new(),
        }
    }

    pub fn push_front(&mut self, elem: T) {
        self.a.push_front(elem);
        self.balance();
    }

    pub fn push_back(&mut self, elem: T) {
        self.b.push_back(elem);
        self.balance();
    }

    pub fn push_middle(&mut self, elem: T) {
        self.a.push_back(elem);
        self.balance();
    }

    pub fn get(&self, at: usize) -> Option<&T> {
        if at < self.a.len() {
            self.a.get(at)
        } else {
            self.b.get(at - self.a.len())
        }
    }

    pub fn len(&self) -> usize {
        self.a.len() + self.b.len()
    }

    pub fn is_empty(&self) -> bool {
        self.a.is_empty() && self.b.is_empty()
    }

    fn balance(&mut self) {
        if self.a.len() > self.b.len() + 1 {
            if let Some(e) = self.a.pop_back() {
                self.b.push_front(e);
            }
        } else if self.a.len() < self.b.len() {
            if let Some(e) = self.b.pop_front() {
                self.a.push_back(e);
            }
        }
    }
}

impl<T> Default for Teque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Teque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index in bounds")
    }
}

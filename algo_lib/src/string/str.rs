use crate::io::{
    input::{Input, Readable},
    output::{Output, Writable},
};
use crate::transparent_wrapper;
use std::{
    fmt::Display,
    io::Write,
    ops::{AddAssign, Deref, DerefMut},
    str::from_utf8_unchecked,
};

transparent_wrapper!(Str = Vec<u8>, derive Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd);

impl Str {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn unwrap(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for Str {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
impl From<&[u8]> for Str {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}
impl<const SIZE: usize> From<&[u8; SIZE]> for Str {
    fn from(value: &[u8; SIZE]) -> Self {
        Self(value.to_vec())
    }
}

impl Readable for Str {
    fn read(input: &mut Input) -> Self {
        Self(input.next_token().unwrap_or_default())
    }
}
impl Writable for Str {
    fn write(&self, output: &mut Output) {
        output.write_all(self).unwrap()
    }
}

impl IntoIterator for Str {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a Str {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> IntoIterator for &'a mut Str {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl FromIterator<u8> for Str {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl AddAssign<&[u8]> for Str {
    fn add_assign(&mut self, rhs: &[u8]) {
        self.0.extend_from_slice(rhs);
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { f.write_str(from_utf8_unchecked(&self.0)) }
    }
}

pub trait StrReader {
    fn read_str(&mut self) -> Str;
    fn read_str_vec(&mut self, size: usize) -> Vec<Str>;
    fn read_line(&mut self) -> Str;
    fn read_line_vec(&mut self, size: usize) -> Vec<Str>;
    fn read_lines(&mut self) -> Vec<Str>;
}
impl StrReader for Input {
    fn read_str(&mut self) -> Str {
        self.read()
    }
    fn read_str_vec(&mut self, size: usize) -> Vec<Str> {
        self.read_vec(size)
    }
    fn read_line(&mut self) -> Str {
        let mut res = Str::new();
        while let Some(b) = self.get() {
            if self.is_eol() {
                break;
            }
            res.push(b);
        }
        res
    }
    fn read_line_vec(&mut self, size: usize) -> Vec<Str> {
        let mut lines = Vec::with_capacity(size);
        for _ in 0..size {
            lines.push(self.read_line());
        }
        lines
    }
    fn read_lines(&mut self) -> Vec<Str> {
        let mut lines = Vec::new();
        while !self.is_exhausted() {
            lines.push(self.read_line());
        }
        if let Some(line) = lines.last() {
            if line.is_empty() {
                lines.pop();
            }
        }
        lines
    }
}

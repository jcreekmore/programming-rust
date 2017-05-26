pub struct Cursor<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    idx: usize,
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(vec: &'a mut Vec<T>) -> Cursor<'a, T> {
        Cursor { vec: vec, idx: 0 }
    }

    pub fn next<'b>(&'b mut self) -> Option<&'b mut T> {
        let idx = self.idx;
        if idx < self.vec.len() {
            self.idx += 1;
        }
        self.vec.get_mut(idx)
    }

    pub fn prev<'b>(&'b mut self) -> Option<&'b mut T> {
        if self.idx > 0 {
            self.idx -= 1;
            self.vec.get_mut(self.idx)
        } else {
            None
        }
    }

    pub fn insert(&mut self, val: T) {
        self.vec.insert(self.idx, val);
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.idx >= self.vec.len() {
            None
        } else {
            Some(self.vec.remove(self.idx))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

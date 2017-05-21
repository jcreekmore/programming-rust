pub struct LinkedList<T> {
    head: Option<Node<T>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T: 'a> {
    current_node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(curr) = self.current_node {
            self.current_node = curr.next
                .as_ref()
                .map(|n| &**n);
            Some(&curr.val)
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    current_node: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        let curr = self.current_node.take();
        if let Some(mut curr) = curr {
            self.current_node = curr.next
                .as_mut()
                .map(|n| &mut **n);
            Some(&mut curr.val)
        } else {
            None
        }
    }
}

pub struct IntoIter<T> {
    current_node: Option<Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let curr = self.current_node.take();
        if let Some(curr) = curr {
            self.current_node = curr.next
                .map(|n| *n);
            Some(curr.val)
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        let head = self.head;
        IntoIter { current_node: head }
    }
}

impl<T> LinkedList<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        let head = self.head.as_ref();
        Iter { current_node: head }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        let head = self.head.as_mut();
        IterMut { current_node: head }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

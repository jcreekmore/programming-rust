pub struct ZipLongest<I, J>
    where I: Iterator,
          J: Iterator
{
    i: I,
    j: J,
}

impl<I, J> Iterator for ZipLongest<I, J>
    where I: Iterator,
          J: Iterator
{
    type Item = (Option<I::Item>, Option<J::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i.next();
        let j = self.j.next();

        if i.is_none() && j.is_none() {
            None
        } else {
            Some((i, j))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

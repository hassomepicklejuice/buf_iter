use std::{collections::VecDeque, num::NonZeroUsize, ops::RangeBounds, slice::SliceIndex};

/// A buffered iterator (or a lazy stack) whose elements are generated from an iterator, and stored in an internal buffer
#[derive(Clone, Debug)]
pub struct BufIter<Iter: Iterator> {
    iter: Iter,
    buf: VecDeque<Iter::Item>,
}

// Public implementation
impl<Iter> BufIter<Iter>
where
    Iter: Iterator,
{
    pub fn new<I: IntoIterator>(iter: I) -> BufIter<I::IntoIter> {
        BufIter {
            iter: iter.into_iter(),
            buf: VecDeque::new(),
        }
    }
    /// Pushes an item to the front of the iterator
    pub fn push(&mut self, item: Iter::Item) {
        self.buf.push_front(item);
    }
    /// Returns the next item in the iterator.
    pub fn pop(&mut self) -> Option<Iter::Item> {
        if self.buf.is_empty() {
            self.iter.next()
        } else {
            self.buf.pop_front()
        }
    }
    /// Returns a reference to the next item in the iterator, without consuming.
    pub fn peek(&mut self, n: usize) -> Option<&Iter::Item> {
        self.prepare_n(n + 1).ok()?;
        self.buf.get(n)
    }
    /// Returns a mutable reference to the next item in the iterator, without consuming.
    pub fn peek_mut(&mut self, n: usize) -> Option<&mut Iter::Item> {
        self.prepare_n(n + 1).ok()?;
        self.buf.get_mut(n)
    }
    /// Returns a reference to a slice of items in the iterator corresponding to the provided range.
    pub fn peek_slice<R>(&mut self, index: R) -> Result<&R::Output, &R::Output>
    where
        R: SliceIndex<[Iter::Item]> + RangeBounds<usize>,
    {
        todo!()
    }
    /// Returns a mutable reference to a slice of items in the iterator corresponding to the provided range.
    pub fn peek_slice_mut<R>(&mut self, index: R) -> Result<&mut R::Output, &mut R::Output>
    where
        R: SliceIndex<[Iter::Item]> + RangeBounds<usize>,
    {
        todo!()
    }
}

impl<Iter> Iterator for BufIter<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<Iter> ExactSizeIterator for BufIter<Iter> where Iter: ExactSizeIterator {}

// Private implementation
impl<Iter> BufIter<Iter>
where
    Iter: Iterator,
{
    fn prepare_n(&mut self, n: usize) -> Result<(), NonZeroUsize> {
        self.buf.reserve(n.saturating_sub(self.buf.len()));
        while self.buf.len() < n {
            let Some(item) = self.iter.next() else {
                break;
            };
            self.buf.push_back(item);
        }
        match NonZeroUsize::new(n.saturating_sub(self.buf.len())) {
            Some(n) => Err(n),
            None => Ok(()),
        }
    }
    fn prepare_all(&mut self) {
        while let Some(item) = self.iter.next() {
            self.buf.push_back(item);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

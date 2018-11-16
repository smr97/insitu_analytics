extern crate itertools;
use itertools::Itertools;
use std::mem::replace;
pub struct Merged<I>
where
    I: Iterator,
    I::Item: Sized + Ord,
{
    original_iterators: Vec<I>,
    heads: Vec<I::Item>,
}

impl<I> Iterator for Merged<I>
where
    I::Item: Sized + Ord,
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let possible_advancing_iterator = self.heads.iter().enumerate().min_by_key(|(_, e)| *e).map(|(index, _)| index);
        if let Some(advancing_iterator) = possible_advancing_iterator {
            let next_head = self.original_iterators[advancing_iterator].next();
            if let Some(next_item) = next_head {
                Some(replace(&mut self.heads[advancing_iterator], next_item))
            } else {
                self.original_iterators.remove(advancing_iterator);
                Some(self.heads.remove(advancing_iterator))
            }
        } else {
            None
        }
    }
    //fn size_hint(&self) -> (usize, Option<usize>) {
    //    self.original_iterators
    //        .iter()
    //        .max_by_key(|inner_iter| inner_iter.size_hint())
    //}
}

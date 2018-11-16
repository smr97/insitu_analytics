extern crate itertools;
use itertools::Itertools;
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
        let temp_option: Option<(usize, &Self::Item)> = {
            self.heads
                .iter()
                .enumerate()
                .min_by_key(|(_, first_elem)| *first_elem)
        };
        let temp_ref = &mut self.heads;
        // if let Some((pos, min_elem)) = temp_option {
        //     temp_ref[pos] = self.original_iterators[pos]
        //         .next()
        //         .unwrap_or_else(|| temp_ref.remove(pos));
        // }
        unimplemented!()
    }
    //fn size_hint(&self) -> (usize, Option<usize>) {
    //    self.original_iterators
    //        .iter()
    //        .max_by_key(|inner_iter| inner_iter.size_hint())
    //}
}

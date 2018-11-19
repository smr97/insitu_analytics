use std::mem::replace;
pub trait MergeTrait: Iterator {
    fn mymerge(self) -> Merged<<Self::Item as IntoIterator>::IntoIter>
    where
        Self: Sized,
        Self::Item: IntoIterator,
        <Self::Item as IntoIterator>::Item: Ord,
    {
        construct_merged(self)
    }
}

fn construct_merged<I>(iterable: I) -> Merged<<I::Item as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
    <<I as IntoIterator>::Item as IntoIterator>::Item: Ord,
{
    let mut vector_of_iterators = Vec::new();
    let mut heads = Vec::new();
    for inner_iterable in iterable.into_iter() {
        let mut inner_iterator = inner_iterable.into_iter();
        let mut option_item = inner_iterator.next();
        if let Some(actual_item) = option_item {
            vector_of_iterators.push(inner_iterator);
            heads.push(actual_item);
        }
    }
    Merged {
        original_iterators: vector_of_iterators,
        heads: heads,
    }
}

impl<I> MergeTrait for I where I: Iterator {}

#[derive(Clone)]
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
        let possible_advancing_iterator = (0..self.heads.len()).min_by_key(|i| &self.heads[*i]);
        //let possible_advancing_iterator = self
        //    .heads
        //    .iter()
        //    .enumerate()
        //    .min_by_key(|(_, e)| *e)
        //    .map(|(index, _)| index);
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
}

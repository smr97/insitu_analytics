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
    let mut vector_of_iterators = Vec::new(); //FIXME: try not to push for the sake of time.
    let mut heads = Vec::new();
    for inner_iter in iterable.into_iter() {
        let mut temp_iter = inner_iter.into_iter();
        let mut option_item = temp_iter.next();
        if let Some(actual_item) = option_item {
            vector_of_iterators.push(temp_iter);
            heads.push(actual_item);
        } else {
            continue;
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
        let possible_advancing_iterator = self
            .heads
            .iter()
            .enumerate()
            .min_by_key(|(_, e)| *e)
            .map(|(index, _)| index);
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

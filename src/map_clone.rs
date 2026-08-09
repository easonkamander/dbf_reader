use fallible_streaming_iterator::FallibleStreamingIterator;

pub struct MapClone<Iter, Func> {
    iter: Iter,
    func: Func,
}

impl<Out, Iter: FallibleStreamingIterator, Func: FnMut(Result<&Iter::Item, Iter::Error>) -> Out>
    Iterator for MapClone<Iter, Func>
{
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().transpose().map(&mut self.func)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub trait WithMapClone: FallibleStreamingIterator + Sized {
    fn map_clone<Out, Func: FnMut(Result<&Self::Item, Self::Error>) -> Out>(
        self,
        func: Func,
    ) -> MapClone<Self, Func> {
        MapClone { iter: self, func }
    }
}

impl<Iter: FallibleStreamingIterator> WithMapClone for Iter {}

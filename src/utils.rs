pub trait IterExt: Iterator {
    fn next_chunk<const N: usize>(
        &mut self,
    ) -> Result<Vec<Self::Item>, std::vec::IntoIter<Self::Item>>
    where
        Self: Sized,
    {
        let mut buf: Vec<Self::Item> = Vec::with_capacity(N);
        for _ in 0..N {
            match self.next() {
                Some(item) => buf.push(item),
                None => {
                    return Err(buf.into_iter());
                }
            }
        }
        Ok(buf)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator {}

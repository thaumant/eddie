
pub trait Zippable {
    type IntoIter;
    fn zip(self) -> Self::IntoIter;
}


impl<T1, T2> Zippable for (T1, T2)
where
    T1: IntoIterator,
    T2: IntoIterator,
{
    type IntoIter = Zipped2<T1, T2>;

    fn zip(self) -> Self::IntoIter {
        let (src1, src2) = self;
        Zipped2::new(src1, src2)
    }
}


impl<T1, T2, T3, T4> Zippable for (T1, T2, T3, T4)
where
    T1: IntoIterator,
    T2: IntoIterator,
    T3: IntoIterator,
    T4: IntoIterator,
{
    type IntoIter = Zipped4<T1, T2, T3, T4>;

    fn zip(self) -> Self::IntoIter {
        let (src1, src2, src3, src4) = self;
        Zipped4::new(src1, src2, src3, src4)
    }
}


pub struct Zipped2<T1, T2>
where
    T1: IntoIterator,
    T2: IntoIterator,
{
    iter1: T1::IntoIter,
    iter2: T2::IntoIter,
}


impl<T1, T2> Zipped2<T1, T2>
where
    T1: IntoIterator,
    T2: IntoIterator,
{
    pub fn new(src1: T1, src2: T2) -> Self {
        let iter1 = src1.into_iter();
        let iter2 = src2.into_iter();
        Self { iter1, iter2 }
    }
}


impl<T1, T2> Iterator for Zipped2<T1, T2>
where
    T1: IntoIterator,
    T2: IntoIterator,
{
    type Item = (T1::Item, T2::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item1 = self.iter1.next()?;
        let item2 = self.iter2.next()?;
        Some((item1, item2))
    }
}


pub struct Zipped4<T1, T2, T3, T4>
where
    T1: IntoIterator,
    T2: IntoIterator,
    T3: IntoIterator,
    T4: IntoIterator,
{
    iter1: T1::IntoIter,
    iter2: T2::IntoIter,
    iter3: T3::IntoIter,
    iter4: T4::IntoIter,
}


impl<T1, T2, T3, T4> Zipped4<T1, T2, T3, T4>
where
    T1: IntoIterator,
    T2: IntoIterator,
    T3: IntoIterator,
    T4: IntoIterator,
{
    pub fn new(src1: T1, src2: T2, src3: T3, src4: T4) -> Self {
        let iter1 = src1.into_iter();
        let iter2 = src2.into_iter();
        let iter3 = src3.into_iter();
        let iter4 = src4.into_iter();
        Self { iter1, iter2, iter3, iter4 }
    }
}


impl<T1, T2, T3, T4> Iterator for Zipped4<T1, T2, T3, T4>
where
    T1: IntoIterator,
    T2: IntoIterator,
    T3: IntoIterator,
    T4: IntoIterator,
{
    type Item = (T1::Item, T2::Item, T3::Item, T4::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item1 = self.iter1.next()?;
        let item2 = self.iter2.next()?;
        let item3 = self.iter3.next()?;
        let item4 = self.iter4.next()?;
        Some((item1, item2, item3, item4))
    }
}

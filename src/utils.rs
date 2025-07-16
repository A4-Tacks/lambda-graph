use std::ops::IndexMut;

pub trait Sign: IndexMut<usize> {
    fn sign(&mut self, i: usize) -> &mut Self::Output;
}
impl<T: Default> Sign for Vec<T> {
    fn sign(&mut self, i: usize) -> &mut Self::Output {
        if i >= self.len() {
            self.resize_with(i+1, Default::default);
        }
        &mut self[i]
    }
}

pub trait MaxTo {
    fn max_to(&mut self, rhs: Self);
}
impl<T: Ord + Copy> MaxTo for T {
    fn max_to(&mut self, rhs: Self) {
        if *self < rhs {
            *self = rhs
        }
    }
}

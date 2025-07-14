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

#[cfg(feature = "swappable")]
extern crate atomic_immut;

#[cfg(feature = "swappable")]
pub use swappable::SwappableFactory;

use std::marker::PhantomData;

#[cfg(feature = "swappable")]
mod swappable;

pub trait Factory {
    type Item;

    fn create(&self) -> Self::Item;
}

#[derive(Debug, Default)]
pub struct DefaultFactory<T>(PhantomData<T>);
impl<T: Default> DefaultFactory<T> {
    pub fn new() -> Self {
        DefaultFactory(PhantomData)
    }
}
impl<T: Default> Factory for DefaultFactory<T> {
    type Item = T;

    fn create(&self) -> Self::Item {
        T::default()
    }
}
impl<T> Clone for DefaultFactory<T> {
    fn clone(&self) -> Self {
        DefaultFactory(PhantomData)
    }
}
unsafe impl<T> Send for DefaultFactory<T> {}
unsafe impl<T> Sync for DefaultFactory<T> {}

#[derive(Debug, Default, Clone)]
pub struct CloneFactory<T>(T);
impl<T: Clone> CloneFactory<T> {
    pub fn new(original: T) -> Self {
        CloneFactory(original)
    }

    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T: Clone> Factory for CloneFactory<T> {
    type Item = T;

    fn create(&self) -> Self::Item {
        self.0.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_factory_works() {
        let f = DefaultFactory::<u8>::new();
        assert_eq!(f.create(), 0);
        assert_eq!(f.clone().create(), 0);
    }

    #[test]
    fn clone_factory_works() {
        let mut f = CloneFactory::new(32);
        assert_eq!(f.get_ref(), &32);
        assert_eq!(f.create(), 32);

        *f.get_mut() = 50;
        assert_eq!(f.create(), 50);
    }
}

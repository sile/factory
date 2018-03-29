use std::sync::Arc;
use atomic_immut::AtomicImmut;

use Factory;

#[derive(Debug, Default)]
pub struct SwappableFactory<T>(Arc<AtomicImmut<T>>);
impl<T: Factory> SwappableFactory<T> {
    pub fn new(inner: T) -> Self {
        SwappableFactory(Arc::new(AtomicImmut::new(inner)))
    }

    pub fn get(&self) -> Arc<T> {
        self.0.load()
    }

    pub fn swap(&self, new: T) -> Arc<T> {
        self.0.swap(new)
    }
}
impl<T: Factory> Factory for SwappableFactory<T> {
    type Item = T::Item;

    fn create(&self) -> Self::Item {
        self.0.load().create()
    }
}
impl<T> Clone for SwappableFactory<T> {
    fn clone(&self) -> Self {
        SwappableFactory(Arc::clone(&self.0))
    }
}

#[cfg(test)]
mod test {
    use {CloneFactory, Factory};
    use super::*;

    #[test]
    fn swappable_factory_works() {
        let f = SwappableFactory::new(CloneFactory::new(32));
        assert_eq!(f.create(), 32);

        f.swap(CloneFactory::new(50));
        assert_eq!(f.create(), 50);
    }
}

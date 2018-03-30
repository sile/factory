use std::sync::Arc;
use atomic_immut::AtomicImmut;

use Factory;

/// `#[cfg(feature = "swappable")]` A `Factory` that allows for swapping inner factories dynamically.
///
/// This use a lock-free data structure for swapping.
///
/// # Examples
///
/// ```
/// use factory::{CloneFactory, Factory, SwappableFactory};
///
/// let f0 = SwappableFactory::new(CloneFactory::new(32));
/// let f1 = f0.clone();
/// assert_eq!(f0.create(), 32);
/// assert_eq!(f1.create(), 32);
///
/// f0.swap(CloneFactory::new(50));
/// assert_eq!(f0.create(), 50);
/// assert_eq!(f1.create(), 50);
/// ```
#[derive(Debug, Default)]
pub struct SwappableFactory<T>(Arc<AtomicImmut<T>>);
impl<T: Factory> SwappableFactory<T> {
    /// Makes a new `SwappableFactory` with the initial inner factory.
    pub fn new(inner: T) -> Self {
        SwappableFactory(Arc::new(AtomicImmut::new(inner)))
    }

    /// Returns the currently used factory.
    pub fn get(&self) -> Arc<T> {
        self.0.load()
    }

    /// Updates inner factory by `new`, and returns old one.
    ///
    /// This operation affects all `SwappableFactory` instances cloned from the original one.
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
    use {CloneFactory, DefaultFactory, Factory};
    use super::*;

    #[test]
    fn swappable_factory_works() {
        let f = SwappableFactory::new(CloneFactory::new(32));
        assert_eq!(f.create(), 32);

        f.swap(CloneFactory::new(50));
        assert_eq!(f.create(), 50);
    }

    #[test]
    fn swappable_box_factory_works() {
        let first: Box<Factory<Item = i32> + 'static> = Box::new(CloneFactory::new(32));
        let f = SwappableFactory::new(first);
        assert_eq!(f.create(), 32);

        let second = Box::new(DefaultFactory::new());
        f.swap(second);
        assert_eq!(f.create(), 0);
    }
}

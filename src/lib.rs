//! This crate provides `Factory` trait and its implementations.
//!
//! The trait makes it possible to create any number of instances of a specific type.
//!
//! # Examples
//!
//! Creates default instances of `u8` type:
//!
//! ```
//! use factory::{DefaultFactory, Factory};
//!
//! let f = DefaultFactory::<u8>::new();
//! assert_eq!(f.create(), 0);
//! assert_eq!(f.create(), 0);
//! ```
#![warn(missing_docs)]

#[cfg(feature = "swappable")]
extern crate atomic_immut;

#[cfg(feature = "swappable")]
pub use swappable::SwappableFactory;

use std::marker::PhantomData;

#[cfg(feature = "swappable")]
mod swappable;

/// This trait allows for creating any number of instances of the `Item` type.
pub trait Factory {
    /// The type of instances created by this factory.
    type Item;

    /// Creates an instance.
    fn create(&self) -> Self::Item;
}
impl<T: ?Sized + Factory> Factory for Box<T> {
    type Item = T::Item;

    fn create(&self) -> Self::Item {
        (**self).create()
    }
}

/// A `Factory` that creates instances using `T::default()` function.
///
/// # Examples
///
/// ```
/// use factory::{DefaultFactory, Factory};
///
/// let f = DefaultFactory::<u8>::new();
/// assert_eq!(f.create(), 0);
/// ```
#[derive(Debug, Default)]
pub struct DefaultFactory<T>(PhantomData<T>);
impl<T: Default> DefaultFactory<T> {
    /// Makes a new `DefaultFactory`.
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

/// A `Factory` that creates instances using `T::clone()` method.
///
/// # Examples
///
/// ```
/// use factory::{CloneFactory, Factory};
///
/// let f = CloneFactory::new(10);
/// assert_eq!(f.create(), 10);
/// ```
#[derive(Debug, Default, Clone)]
pub struct CloneFactory<T>(T);
impl<T: Clone> CloneFactory<T> {
    /// Makes a new `CloneFactory`.
    ///
    /// The instances the factory creates are copied from `original`.
    pub fn new(original: T) -> Self {
        CloneFactory(original)
    }

    /// Returns a reference to the original instance.
    pub fn get_ref(&self) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the original instance.
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

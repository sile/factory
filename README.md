factory
=======

[![factory](http://meritbadge.herokuapp.com/factory)](https://crates.io/crates/factory)
[![Documentation](https://docs.rs/factory/badge.svg)](https://docs.rs/factory)
[![Build Status](https://travis-ci.org/sile/factory.svg?branch=master)](https://travis-ci.org/sile/factory)
[![Code Coverage](https://codecov.io/gh/sile/factory/branch/master/graph/badge.svg)](https://codecov.io/gh/sile/factory/branch/master)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This crate provides `Factory` trait and its implementations.

The trait makes it possible to create any number of instances of a specific type.

[Documentation](https://docs.rs/factory)

Examples
--------

Creates default instances of `u8` type:

```
use factory::{DefaultFactory, Factory};

let f = DefaultFactory::<u8>::new();
assert_eq!(f.create(), 0);
```

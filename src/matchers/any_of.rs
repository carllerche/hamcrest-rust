// Copyright 2017 Flier Lu
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{self, Display};
use std::marker::PhantomData;

use core::*;

pub struct AnyOf<T, M>(M, PhantomData<T>);

pub fn any_of<T, M>(matchers: M) -> AnyOf<T, M> {
    AnyOf(matchers, PhantomData)
}

#[macro_export]
macro_rules! any_of {
    ($( $arg:expr ),*) => ($crate::matchers::any_of::any_of(($( $arg ),*)))
}

impl<T, M0, M1> Display for AnyOf<T, (M0, M1)>
where
    M0: Display,
    M1: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref m0, ref m1) = self.0;

        write!(f, "any of ({}, {})", m0, m1)
    }
}

impl<T, M0, M1> Matcher<T> for AnyOf<T, (M0, M1)>
where
    T: Clone,
    M0: Matcher<T>,
    M1: Matcher<T>,
{
    fn matches(&self, actual: T) -> MatchResult {
        let (ref m0, ref m1) = self.0;

        m0.matches(actual.clone()).or_else(
            |_| m1.matches(actual.clone()),
        )
    }
}

impl<T, M0, M1, M2> Display for AnyOf<T, (M0, M1, M2)>
where
    M0: Display,
    M1: Display,
    M2: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref m0, ref m1, ref m2) = self.0;

        write!(f, "any of ({}, {}, {})", m0, m1, m2)
    }
}

impl<T, M0, M1, M2> Matcher<T> for AnyOf<T, (M0, M1, M2)>
where
    T: Clone,
    M0: Matcher<T>,
    M1: Matcher<T>,
    M2: Matcher<T>,
{
    fn matches(&self, actual: T) -> MatchResult {
        let (ref m0, ref m1, ref m2) = self.0;

        m0.matches(actual.clone())
            .or_else(|_| m1.matches(actual.clone()))
            .or_else(|_| m2.matches(actual.clone()))
    }
}

impl<T, M0, M1, M2, M3> Display for AnyOf<T, (M0, M1, M2, M3)>
where
    M0: Display,
    M1: Display,
    M2: Display,
    M3: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref m0, ref m1, ref m2, ref m3) = self.0;

        write!(f, "any of ({}, {}, {}, {})", m0, m1, m2, m3)
    }
}

impl<T, M0, M1, M2, M3> Matcher<T> for AnyOf<T, (M0, M1, M2, M3)>
where
    T: Clone,
    M0: Matcher<T>,
    M1: Matcher<T>,
    M2: Matcher<T>,
    M3: Matcher<T>,
{
    fn matches(&self, actual: T) -> MatchResult {
        let (ref m0, ref m1, ref m2, ref m3) = self.0;

        m0.matches(actual.clone())
            .or_else(|_| m1.matches(actual.clone()))
            .or_else(|_| m2.matches(actual.clone()))
            .or_else(|_| m3.matches(actual.clone()))
    }
}

impl<T, M0, M1, M2, M3, M4> Display for AnyOf<T, (M0, M1, M2, M3, M4)>
where
    M0: Display,
    M1: Display,
    M2: Display,
    M3: Display,
    M4: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref m0, ref m1, ref m2, ref m3, ref m4) = self.0;

        write!(f, "any of ({}, {}, {}, {}, {})", m0, m1, m2, m3, m4)
    }
}

impl<T, M0, M1, M2, M3, M4> Matcher<T> for AnyOf<T, (M0, M1, M2, M3, M4)>
where
    T: Clone,
    M0: Matcher<T>,
    M1: Matcher<T>,
    M2: Matcher<T>,
    M3: Matcher<T>,
    M4: Matcher<T>,
{
    fn matches(&self, actual: T) -> MatchResult {
        let (ref m0, ref m1, ref m2, ref m3, ref m4) = self.0;

        m0.matches(actual.clone())
            .or_else(|_| m1.matches(actual.clone()))
            .or_else(|_| m2.matches(actual.clone()))
            .or_else(|_| m3.matches(actual.clone()))
            .or_else(|_| m4.matches(actual.clone()))
    }
}

impl<T, M0, M1, M2, M3, M4, M5> Display for AnyOf<T, (M0, M1, M2, M3, M4, M5)>
where
    M0: Display,
    M1: Display,
    M2: Display,
    M3: Display,
    M4: Display,
    M5: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref m0, ref m1, ref m2, ref m3, ref m4, ref m5) = self.0;

        write!(f, "any of ({}, {}, {}, {}, {}, {})", m0, m1, m2, m3, m4, m5)
    }
}

impl<T, M0, M1, M2, M3, M4, M5> Matcher<T> for AnyOf<T, (M0, M1, M2, M3, M4, M5)>
where
    T: Clone,
    M0: Matcher<T>,
    M1: Matcher<T>,
    M2: Matcher<T>,
    M3: Matcher<T>,
    M4: Matcher<T>,
    M5: Matcher<T>,
{
    fn matches(&self, actual: T) -> MatchResult {
        let (ref m0, ref m1, ref m2, ref m3, ref m4, ref m5) = self.0;

        m0.matches(actual.clone())
            .or_else(|_| m1.matches(actual.clone()))
            .or_else(|_| m2.matches(actual.clone()))
            .or_else(|_| m3.matches(actual.clone()))
            .or_else(|_| m4.matches(actual.clone()))
            .or_else(|_| m5.matches(actual.clone()))
    }
}

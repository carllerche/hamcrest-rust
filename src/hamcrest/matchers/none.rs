use std::fmt;
use std::marker::PhantomData;

use {success, Matcher, MatchResult};

pub struct IsNone<T> {
  marker: PhantomData<T>,
}

impl<T> fmt::Display for IsNone<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "none")
  }
}

impl<T: fmt::Debug> Matcher<Option<T>> for IsNone<T> {
  fn matches(&self, actual: Option<T>) -> MatchResult {
    match actual {
        Some(s) => Err(format!("was Some({:?})", s)),
        None => success(),
    }
  }
}

pub fn none<T>() -> IsNone<T> {
  IsNone { marker: PhantomData }
}

#[cfg(test)]
mod test {
  use {assert_that,is,is_not,none};

  #[test]
  fn test_none_is_none() {
    assert_that(None, is(none::<int>()));
    assert_that(Some(1), is_not(none::<int>()));
  }
}

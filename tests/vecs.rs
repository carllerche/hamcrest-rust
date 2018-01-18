// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest;

mod vecs {

    use hamcrest::prelude::*;

    #[test]
    fn vec_contains_all_of() {
        assert_that!(&vec![1, 2, 3], contains_all_of(vec![1, 2]));
        assert_that!(&vec![1, 2, 3], not(contains_all_of(vec![4])));
        assert_that!(&vec![1, 2, 3], does_not(contain_all_of(vec![4])));
    }

    #[test]
    fn vec_contains_all_of_exactly() {
        assert_that!(&vec![1, 2, 3], contains_all_of(vec![1, 2, 3]).exactly());
        assert_that!(&vec![1, 2, 3], not(contains_all_of(vec![1, 2]).exactly()));
    }

    #[test]
    fn it_contains_all_of_elements_in_order() {
        assert_that!(&vec![1, 2, 3], contains_all_of(vec![1, 2]).in_order());
    }

    #[test]
    fn it_does_not_contain_elements_in_order() {
        assert_that!(&vec![1, 2, 3], not(contains_all_of(vec![1, 3]).in_order()));
    }

    #[test]
    #[should_panic]
    fn it_unsuccessfully_contains_all_of_elements_in_order() {
        assert_that!(&vec![1, 2, 3], contains_all_of(vec![1, 3]).in_order());
    }

    #[test]
    #[should_panic]
    fn it_unsuccessfully_does_not_contain_elements_in_order() {
        assert_that!(&vec![1, 2, 3], not(contains_all_of(vec![2, 3]).in_order()));
    }

    #[test]
    fn vec_of_len() {
        assert_that!(&vec![1, 2, 3], of_len(3));
        assert_that!(&vec![1, 2, 3], is(of_len(3)));
    }

}

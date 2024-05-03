use std::ops::Range;

use chumsky::{error::Simple, Error};

pub fn custom_span(msg: &str) -> impl Fn(Range<usize>) -> Simple<char> + '_ {
    move |e| Simple::custom(e, msg)
}

pub fn merge_custom_in_simple(msg: &str) -> impl Fn(Simple<char>) -> Simple<char> + '_ {
    move |e| {
        let custom = Simple::custom(e.span(), msg);
        Simple::merge(e, custom)
    }
}

pub fn custom_simple(msg: &str) -> impl Fn(Simple<char>) -> Simple<char> + '_ {
    move |e| Simple::custom(e.span(), msg)
}

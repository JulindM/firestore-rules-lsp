use std::ops::Range;

use chumsky::error::Simple;

pub type SimpleCharError = Simple<char>;

pub trait Spannable {
    fn get_span(&self) -> Range<usize>;
}

impl Spannable for SimpleCharError {
    fn get_span(&self) -> Range<usize> {
        self.span()
    }
}

impl Spannable for Range<usize> {
    fn get_span(&self) -> Range<usize> {
        self.clone()
    }
}

pub fn gen_error<S: Spannable>(msg: &str) -> impl Fn(S) -> SimpleCharError + '_ {
    move |e| Simple::custom(e.get_span(), msg)
}

use std::ops::Range;

use chumsky::error::{Simple, SimpleReason};

pub type SimpleCharError = Simple<char>;

pub trait SpannableMessage {
    fn get_span(&self) -> Range<usize>;
    fn get_message(&self) -> String;
    fn get_label(&self) -> String;
}

impl SpannableMessage for SimpleCharError {
    fn get_span(&self) -> Range<usize> {
        self.span()
    }

    fn get_message(&self) -> String {
        match self.reason() {
            SimpleReason::Custom(s) => s.to_string(),
            SimpleReason::Unexpected => format!(
                "expected {:?} but found a {:?}",
                self.expected()
                    .into_iter()
                    .map(|c| c)
                    .collect::<Vec<&Option<char>>>(),
                self.found().to_owned()
            )
            .to_owned(),

            SimpleReason::Unclosed { span, delimiter } => {
                format!("Unclosed delimiter {:?} at {}", span, delimiter).to_string()
            }
        }
    }

    fn get_label(&self) -> String {
        self.label().unwrap_or("Unknown").to_owned()
    }
}

impl SpannableMessage for Range<usize> {
    fn get_span(&self) -> Range<usize> {
        self.clone()
    }

    fn get_message(&self) -> String {
        "Custom".to_owned()
    }

    fn get_label(&self) -> String {
        "Unlabeled".to_owned()
    }
}

pub fn gen_error<S: SpannableMessage>(msg: &str) -> impl Fn(S) -> SimpleCharError + '_ {
    move |e| {
        Simple::custom(
            e.get_span(),
            format!("({}: {} -> {})", e.get_label(), msg, e.get_message()),
        )
    }
}

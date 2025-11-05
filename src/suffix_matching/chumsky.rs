use chumsky::input::IterInput;
use chumsky::prelude::*;

#[derive(Debug, Clone)]
struct RevSource<'s> {
    data: &'s [u8],
    len: usize,
    offset: usize,
}

impl<'s> RevSource<'s> {
    pub fn new(data: &'s [u8]) -> Self {
        let len = data.len();
        let offset = 0;
        Self { data, len, offset }
    }

    fn eoi(&self) -> SimpleSpan {
        SimpleSpan::new((), self.len..self.len)
    }
}

impl Iterator for RevSource<'_> {
    type Item = (u8, SimpleSpan);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.len {
            return None;
        }

        let index = self.len - self.offset - 1;
        let span = self.offset..self.offset;
        self.offset += 1;

        Some((self.data[index], SimpleSpan::new((), span)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = self.len - self.offset;
        (rest, Some(rest))
    }
}

type RevInput<'s> = IterInput<RevSource<'s>, SimpleSpan>;
type RevParser<'s, O> = Boxed<'s, 's, RevInput<'s>, O>;

pub struct Matcher<'a> {
    parser: RevParser<'a, ()>,
}

impl<'a> Matcher<'a> {
    pub fn new(suffixes: &[&str]) -> Self {
        let mut choices = vec![];

        for ext in suffixes {
            let mut suffix = ext.as_bytes().to_vec();
            suffix.reverse();

            choices.push(just(suffix));
        }

        let parser = choice(choices).ignored().lazy().boxed();

        Self { parser }
    }

    pub fn matches(&self, input: &'a str) -> bool {
        let src = RevSource::new(input.as_bytes());
        let eoi = src.eoi();
        let res = self.parser.parse(RevInput::new(src, eoi));
        !res.has_errors()
    }
}

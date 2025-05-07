use yada::builder::DoubleArrayBuilder;
use yada::DoubleArray;

pub struct Matcher {
    trie: DoubleArray<Vec<u8>>,
}

impl Matcher {
    pub fn new(suffixes: &[&str]) -> Self {
        let mut keyset = suffixes
            .iter()
            .map(|x| x.as_bytes().iter().copied().rev().collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();

        keyset.sort();
        keyset.dedup();

        let keyset = keyset
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i as u32))
            .collect::<Vec<_>>();

        let bytes = DoubleArrayBuilder::build(&keyset).unwrap();
        let trie = DoubleArray::new(bytes);

        Self { trie }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut key = input.as_bytes().to_vec();
        key.reverse();

        let matched = self.trie.common_prefix_search(&key).next();
        matched.is_some()
    }
}

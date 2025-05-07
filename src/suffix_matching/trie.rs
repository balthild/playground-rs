use trie_rs::{Trie, TrieBuilder};

pub struct Matcher {
    trie: Trie<u8>,
}

impl Matcher {
    pub fn new(suffixes: &[&str]) -> Self {
        let mut builder = TrieBuilder::new();

        for suffix in suffixes {
            let mut key = suffix.as_bytes().to_vec();
            key.reverse();
            builder.push(key);
        }

        let trie = builder.build();

        Self { trie }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut key = input.as_bytes().to_vec();
        key.reverse();

        let matched: Option<Vec<u8>> = self.trie.common_prefix_search(&key).next();
        matched.is_some()
    }
}

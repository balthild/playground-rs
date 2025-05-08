use ptrie::Trie;

pub struct Matcher {
    trie: Trie<u8, ()>,
}

impl Matcher {
    pub fn new(suffixes: &[&str]) -> Self {
        let mut trie = Trie::new();

        for suffix in suffixes {
            let mut key = suffix.as_bytes().to_vec();
            key.reverse();
            trie.insert(key.into_iter(), ());
        }

        Self { trie }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut key = input.as_bytes().to_vec();
        key.reverse();

        let matched = self.trie.find_longest_prefix(key.into_iter());
        matched.is_some()
    }
}

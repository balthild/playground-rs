pub mod chumsky;
pub mod glob;
pub mod ptrie;
pub mod simple;
pub mod trie;
pub mod yada;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    #[bench]
    fn bench_simple_matcher(b: &mut Bencher) {
        let matcher = super::simple::Matcher::new(&get_suffixes());

        assert!(matcher.matches("foo.tar.gz"));
        assert!(!matcher.matches("foo.bar"));

        b.iter(|| matcher.matches(black_box("foo.tar.gz")));
        b.iter(|| matcher.matches(black_box("foo.bar")));
    }

    #[bench]
    fn bench_glob_matcher(b: &mut Bencher) {
        let matcher = super::simple::Matcher::new(&get_suffixes());

        assert!(matcher.matches("foo.tar.gz"));
        assert!(!matcher.matches("foo.bar"));

        b.iter(|| matcher.matches(black_box("foo.tar.gz")));
        b.iter(|| matcher.matches(black_box("foo.bar")));
    }

    #[bench]
    fn bench_trie_matcher(b: &mut Bencher) {
        let matcher = super::trie::Matcher::new(&get_suffixes());

        assert!(matcher.matches("foo.tar.gz"));
        assert!(!matcher.matches("foo.bar"));

        b.iter(|| matcher.matches(black_box("foo.tar.gz")));
        b.iter(|| matcher.matches(black_box("foo.bar")));
    }

    #[bench]
    fn bench_ptrie_matcher(b: &mut Bencher) {
        let matcher = super::ptrie::Matcher::new(&get_suffixes());

        assert!(matcher.matches("foo.tar.gz"));
        assert!(!matcher.matches("foo.bar"));

        b.iter(|| matcher.matches(black_box("foo.tar.gz")));
        b.iter(|| matcher.matches(black_box("foo.bar")));
    }

    #[bench]
    fn bench_yada_matcher(b: &mut Bencher) {
        let matcher = super::yada::Matcher::new(&get_suffixes());

        assert!(matcher.matches("foo.tar.gz"));
        assert!(!matcher.matches("foo.bar"));

        b.iter(|| matcher.matches(black_box("foo.tar.gz")));
        b.iter(|| matcher.matches(black_box("foo.bar")));
    }

    #[bench]
    fn bench_chumsky_matcher(b: &mut Bencher) {
        let matcher = super::chumsky::Matcher::new(&get_suffixes());

        assert!(matcher.matches("foo.tar.gz"));
        assert!(!matcher.matches("foo.bar"));

        b.iter(|| matcher.matches(black_box("foo.tar.gz")));
        b.iter(|| matcher.matches(black_box("foo.bar")));
    }

    fn get_suffixes() -> Vec<&'static str> {
        vec![
            ".foo.bar",
            ".foobar",
            ".png",
            ".jpg",
            ".jpeg",
            ".gif",
            ".svg",
            ".ico",
            ".webp",
            ".bmp",
            ".tiff",
            ".tif",
            ".psd",
            ".ai",
            ".eps",
            ".pdf",
            ".doc",
            ".docx",
            ".docm",
            ".rtf",
            ".xls",
            ".xlsx",
            ".csv",
            ".ppt",
            ".pptx",
            ".odt",
            ".ods",
            ".odp",
            ".html",
            ".htm",
            ".mht",
            ".mhtml",
            ".css",
            ".js",
            ".xml",
            ".json",
            ".jsonld",
            ".json5",
            ".yaml",
            ".yml",
            ".toml",
            ".txt",
            ".mp3",
            ".wav",
            ".mp4",
            ".avi",
            ".mkv",
            ".mpg",
            ".mpeg",
            ".wmv",
            ".flv",
            ".mov",
            ".dat",
            ".zip",
            ".7z",
            ".tar.gz",
            ".tar.bz2",
            ".tar.lzma",
            ".tar.xz",
            ".rar",
            ".exe",
            ".dmg",
            ".iso",
            ".deb",
            ".rpm",
            ".apk",
            ".jar",
            ".war",
            ".xaml",
        ]
    }
}

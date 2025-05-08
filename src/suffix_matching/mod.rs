pub mod chumsky;
pub mod glob;
pub mod ptrie;
pub mod simple;
pub mod triers;
pub mod yada;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    macro_rules! bench_matcher {
        ($b:ident, $t:ty, $few:expr) => {
            let matcher = <$t>::new(&get_suffixes($few));

            assert!(matcher.matches("foo.tar.gz"));
            assert!(!matcher.matches("foo.bar"));

            $b.iter(|| matcher.matches(black_box("foo.tar.gz")));
            $b.iter(|| matcher.matches(black_box("foo.bar")));
        };
    }

    #[bench]
    fn few_simple(b: &mut Bencher) {
        bench_matcher!(b, super::simple::Matcher, true);
    }

    #[bench]
    fn many_simple(b: &mut Bencher) {
        bench_matcher!(b, super::simple::Matcher, false);
    }

    #[bench]
    fn few_glob(b: &mut Bencher) {
        bench_matcher!(b, super::glob::Matcher, true);
    }

    #[bench]
    fn many_glob(b: &mut Bencher) {
        bench_matcher!(b, super::glob::Matcher, false);
    }

    #[bench]
    fn few_trie(b: &mut Bencher) {
        bench_matcher!(b, super::triers::Matcher, true);
    }

    #[bench]
    fn many_trie(b: &mut Bencher) {
        bench_matcher!(b, super::triers::Matcher, false);
    }

    #[bench]
    fn few_ptrie(b: &mut Bencher) {
        bench_matcher!(b, super::ptrie::Matcher, true);
    }

    #[bench]
    fn many_ptrie(b: &mut Bencher) {
        bench_matcher!(b, super::ptrie::Matcher, false);
    }

    #[bench]
    fn few_yada(b: &mut Bencher) {
        bench_matcher!(b, super::yada::Matcher, true);
    }

    #[bench]
    fn many_yada(b: &mut Bencher) {
        bench_matcher!(b, super::yada::Matcher, false);
    }

    #[bench]
    fn few_chumsky(b: &mut Bencher) {
        bench_matcher!(b, super::chumsky::Matcher, true);
    }

    #[bench]
    fn many_chumsky(b: &mut Bencher) {
        bench_matcher!(b, super::chumsky::Matcher, false);
    }

    fn get_suffixes(few: bool) -> Vec<&'static str> {
        if few {
            return vec![".foo.bar", ".foobar", ".tar.gz"];
        }

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

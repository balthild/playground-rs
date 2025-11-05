pub mod chumsky;
pub mod glob;
pub mod ptrie;
pub mod simple;
pub mod simple2;
pub mod triers;
pub mod yada;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    macro_rules! bench_matcher {
        ($name:ident) => {
            bench_matcher!(few_, $name, get_suffixes(true));
            bench_matcher!(many_, $name, get_suffixes(false));
        };
        ($prefix:ident, $name:ident, $haystack:expr) => {
            #[bench]
            fn ${concat(create_, $prefix, $name)}(b: &mut Bencher) {
                b.iter(|| super::$name::Matcher::new(black_box($haystack)));
            }

            #[bench]
            fn ${concat(match_best_, $prefix, $name)}(b: &mut Bencher) {
                let matcher = super::$name::Matcher::new($haystack);
                assert!(matcher.matches("foo.tar.gz"));
                b.iter(|| matcher.matches(black_box("foo.tar.gz")));
            }

            #[bench]
            fn ${concat(match_worst_, $prefix, $name)}(b: &mut Bencher) {
                let matcher = super::$name::Matcher::new($haystack);
                assert!(!matcher.matches("foo.bar"));
                b.iter(|| matcher.matches(black_box("foo.bar")));
            }
        };
    }

    bench_matcher!(simple);
    bench_matcher!(simple2);
    bench_matcher!(glob);
    bench_matcher!(triers);
    bench_matcher!(ptrie);
    bench_matcher!(yada);
    bench_matcher!(chumsky);

    const fn get_suffixes(few: bool) -> &'static [&'static str] {
        if few {
            return &[".foo.bar", ".foobar", ".tar.gz"];
        }

        &[
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

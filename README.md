# playground

A place to do experiments with Rust.

## suffix_matching

Different implementations for matching string suffixes, e.g., file extensions.

```
cargo bench suffix_matching
```

### Conclusion

`yada` and `ptrie` are the fastest. `trie-rs` is unexpectedly slow. `glob-match` is the slowest.

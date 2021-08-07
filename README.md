# Chearmyp Abstract Source
An abstraction of sources that can be processed by lexer and parser, and contained by different structures.

## Installation
Add it to the dependencies:
```
chearmyp_abstract_source = { git = "http://chearmyp.local/chearmyp_abstract_source", tag = "v0.1.1" }
```

You may also activate all the features:
```
[dependencies.chearmyp_abstract_source]
git = "http://chearmyp.local/chearmyp_abstract_source"
tag = "v0.1.1"
features = ["no_std", "str_source", "slice_u8_source", "vec_source_collection"]
```

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

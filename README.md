# Abstract Chearmyp Source
An abstraction of sources that can be processed by lexer and parser, and contained by different structures.

## Installation
Add it to the dependencies:
```
abstract_chearmyp_source = { git = "http://chearmyp.local/abstract_chearmyp_source", tag = "v0.3.0" }
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_source]
git = "http://chearmyp.local/abstract_chearmyp_source"
tag = "v0.3.0"
features = ["no_std", "str_source", "slice_u8_source", "vec_source_collection"]
```

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

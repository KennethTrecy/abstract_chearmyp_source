# Chearmyp Abstract Source
An abstraction of sources that can be processed by lexer and parser, and contained by different structures.

## Installation
Add it to the dependencies:
```
chearmyp_abstract_source = { git = "http://chearmyp.local/chearmyp_abstract_source", tag = "v0.1.0" }
```

You may also activate all the features:
```
[dependencies.chearmyp_abstract_source]
git = "http://chearmyp.local/chearmyp_abstract_source"
tag = "v0.1.0"
features = ["str_source", "slice_u8_source"]
```

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

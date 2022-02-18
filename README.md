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

## Origin
The repository was based from [`filled_bare_metal`] branch of [Feo Template].

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template

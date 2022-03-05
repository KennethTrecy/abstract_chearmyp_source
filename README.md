# Abstract Chearmyp Source
An abstraction of sources that can be processed by lexer and parser, and contained by different data structures.

Initially intended for Chearmyp lexer and parser but it can be used to other projects as well.

## Installation
Add it to the dependencies:
```
[dependencies.abstract_chearmyp_source]
git = "https://github.com/KennethTrecy/abstract_chearmyp_source"
tag = "v0.6.0"
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_source]
git = "https://github.com/KennethTrecy/abstract_chearmyp_source"
tag = "v0.6.0"
features = ["no_std", "str_source", "slice_u8_source", "vec_source_collection", "str_comparable_to_u8", "str_comparable_to_str", "slice_u8_comparable_to_u8", "slice_u8_comparable_to_str"]
```

## Origin
Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

## Author
Abstract Chearmyp Source was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template

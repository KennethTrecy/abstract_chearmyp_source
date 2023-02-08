[![Library Tests](https://img.shields.io/github/actions/workflow/status/KennethTrecy/abstract_chearmyp_source/library.yml?style=for-the-badge)](https://github.com/KennethTrecy/abstract_chearmyp_source/actions/workflows/library.yml)
![GitHub lines](https://img.shields.io/github/license/KennethTrecy/abstract_chearmyp_source?style=for-the-badge)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KennethTrecy/abstract_chearmyp_source?style=for-the-badge&display_name=tag&sort=semver)
![GitHub closed issues count](https://img.shields.io/github/issues-closed/KennethTrecy/abstract_chearmyp_source?style=for-the-badge)
![GitHub pull request count](https://img.shields.io/github/issues-pr-closed/KennethTrecy/abstract_chearmyp_source?style=for-the-badge)
![Commits since latest version](https://img.shields.io/github/commits-since/KennethTrecy/abstract_chearmyp_source/latest?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/KennethTrecy/abstract_chearmyp_source?style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/repo-size/KennethTrecy/abstract_chearmyp_source?style=for-the-badge)

# Abstract Chearmyp Source
An abstraction of sources that can be processed by lexer and parser, and contained by different data structures.

Initially intended for Chearmyp lexer and parser but it can be used to other projects as well.

## Installation
Add it to the dependencies:
```
[dependencies.abstract_chearmyp_source]
git = "https://github.com/KennethTrecy/abstract_chearmyp_source"
tag = "v1.0.0"
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_source]
git = "https://github.com/KennethTrecy/abstract_chearmyp_source"
tag = "v1.0.0"
features = ["no_std", "str_source", "slice_u8_source", "vec_source_collection", "str_comparable_to_u8", "str_comparable_to_str", "slice_u8_comparable_to_u8", "slice_u8_comparable_to_str"]
```

## Origin
Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Usage

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

### Initialization
If you want to contribute, this repository should be initialized to adhere in [Conventional Commits specification] for organize
commits and automated generation of change log.

#### Prerequisites
- [Node.js and NPM]
- [pnpm] (optional)

#### Instructions
By running the command below, all your commits will be linted to follow the [Conventional Commits
specification].
```
$ npm install
```

Or if you have installed [pnpm], run the following command:
```
$ pnpm install
```

To generate the change log automatically, run the command below:
```
$ npx changelogen --from=[tag name or branch name or commit itself] --to=master
```

## Notes

### License
The repository is licensed under [MIT].

### Want to contribute?
Read the [contributing guide] for different ways to contribute in the project.

### Author
Coded by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[MIT]: https://github.com/KennethTrecy/abstract_chearmyp_source/blob/master/LICENSE
[Node.js and NPM]: https://nodejs.org/en/
[pnpm]: https://pnpm.io/installation
[Conventional Commits specification]: https://www.conventionalcommits.org/en/v1.0.0/
[contributing guide]: ./CONTRIBUTING.md

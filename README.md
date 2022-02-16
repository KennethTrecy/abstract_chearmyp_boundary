# Abstract Chearmyp Boundary
An abstraction of address boundaries on a source. Initially intended to work with [Abstract Chearmyp Source] but it can be used to other applications/packages as well.

## Installation
Add it to the dependencies:
```
[dependencies.abstract_chearmyp_boundary]
git = "https://github.com/KennethTrecy/abstract_chearmyp_boundary"
tag = "v0.1.0"
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_boundary]
git = "http://chearmyp.local/abstract_chearmyp_boundary"
tag = "v0.1.0"
features = ["no_std", "range_boundary", "vec_boundary_collection"]
```

## Origin
The repository was based from [`filled_toml`] branch of [Feo Template].

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

### Author
Coded by Kenneth Trecy Tobias.

[`filled_toml`]: https://github.com/KennethTrecy/feo_template/tree/filled_toml
[Feo Template]: https://github.com/KennethTrecy/feo_template

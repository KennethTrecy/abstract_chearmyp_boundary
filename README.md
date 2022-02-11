# Abstract Chearmyp Boundary
An abstraction of address boundaries on a source. Initially intended to work with [Abstract Chearmyp Source] but it can be used to other applications/packages as well.

## Installation
Add it to the dependencies:
```
abstract_chearmyp_boundary = { git = "http://chearmyp.local/abstract_chearmyp_boundary", tag = "v0.1.0" }
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_boundary]
git = "http://chearmyp.local/abstract_chearmyp_boundary"
tag = "v0.1.0"
features = ["no_std", "range_boundary", "vec_boundary_collection"]
```

### Documentation
Run the following on the console:
```
cargo doc --all-features --open
```

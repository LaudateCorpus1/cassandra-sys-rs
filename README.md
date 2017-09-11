[![Build Status](https://travis-ci.org/Metaswitch/cassandra-sys-rs.svg?branch=master)](https://travis-ci.org/Metaswitch/cassandra-sys-rs)
[![Current Version](http://img.shields.io/crates/v/cassandra-cpp-sys.svg)](https://crates.io/crates/cassandra-cpp-sys)
[![License](https://img.shields.io/github/license/Metaswitch/cassandra-sys-rs.svg)](#license)

# cassandra-cpp-sys

This is a maintained Rust project that provides a low-level binding of the
DataStax cpp driver at https://github.com/datastax/cpp-driver/ .
It is mostly autogenerated.

This project also includes a fairly complete set of examples equivalent to the ones in the C++ repository.

It is quite possible to use this crate directly from your Rust code, but it will mean littering unsafe all over the place.
Instead it is recommended that you use the safe wrapper of this interface: [cassandra-cpp](https://github.com/Metaswitch/cassandra-rs).

[Documentation (crates.io)](https://docs.rs/cassandra-cpp-sys).


## Getting started

For the wrapper to work, you must first have installed the datastax-cpp driver.
Follow the steps in the
[cpp driver docs](https://github.com/datastax/cpp-driver/tree/master/topics#installation)
to do so. Pre-built packages are available for most platforms.

Make sure that the driver (specifically `libcassandra_static.a` and `libcassandra.so`) are in your `/usr/local/lib64/` directory.

You can use this crate from cargo with

```toml
    [dependencies]
    cassandra-cpp-sys = "0.11"
```

## License

This code is open source, licensed under the Apache License Version 2.0 as
described in [`LICENSE`](LICENSE).


## Contributing

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for details on how to contribute
to this project.


## Compilation

You must have the DataStax driver installed on your system in order to build this crate.

By default, `/usr/lib`, `/usr/local/lib64`, and `/usr/local/lib` are added to the linker search path.

A semicolon separated list of additional directories to add to the linker search path may be specified through the `CASSANDRA_SYS_LIB_PATH` environment variable.

## Autogeneration

The file `cassandra.rs` is autogenerated as follows:


```
$ bindgen --no-layout-tests --blacklist-type=max_align_t --output=src/cassandra.rs cassandra.h
```

The autogenerated code is formatted with `rustfmt`.


## History

This project was forked from [cassandra-sys](https://github.com/tupshin/cassandra-sys-rs), which was no longer being maintained.

### 0.11.0 (2017-09-11)

- Regenerate using latest bindgen. Some type incompatibilities are possible, so bumping version as a precaution.

### 0.10.0 (2017-08-02)

- Remove extraneous dependencies (e.g., log).
- Move examples to be proper examples.
- Remove use of error-chain; this is inappropriate for a -sys crate.
- Fix some warnings.
- Correct names of `cass_error_result_received`/`_required`.

### Historical note regarding version 0.9.0

At one point some development was done on preparing a version 0.9.0
using a newer version of bindgen, but the work was not completed or released. See the
[version-0.9](https://github.com/Metaswitch/cassandra-sys-rs/tree/version-0.9) branch
to follow that development. In due course this may be merged to master, but for the moment we recommend you use the
released versions.

### 0.8.8 (2017-06-29)

- Fork crate.
- Remove unused method `cass_cluster_set_queue_size_log`.

### 0.8.7 (2016-12-15)

- (Pre-fork version.)

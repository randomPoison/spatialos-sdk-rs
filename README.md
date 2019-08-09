# SpatialOS SDK for Rust

[![Build Status](https://travis-ci.org/jamiebrynes7/spatialos-sdk-rs.svg?branch=master)](https://travis-ci.org/jamiebrynes7/spatialos-sdk-rs) [![dependency status](https://deps.rs/repo/github/jamiebrynes7/spatialos-sdk-rs/status.svg)](https://deps.rs/repo/github/jamiebrynes7/spatialos-sdk-rs) ![Rustc Version](https://img.shields.io/badge/rustc-1.34-blue.svg)


> This is an **unofficial**, **unsupported**, and **untested** integration of the [SpatialOS SDK C API bindings](https://docs.improbable.io/reference/13.3/capi/introduction) with Rust. Improbable does not officially support Rust as a worker language.

## Requirements

1. Rust v1.34
2. A [SpatialOS account](https://www.improbable.io/get-spatialos) 

## Setup

1. Clone this repository.
2. Install cargo-spatial: `cargo install --path ./cargo-spatial --force`
3. Set the `SPATIAL_LIB_DIR` environment variable to the location of the dependencies: `export SPATIAL_LIB_DIR=$(pwd)/dependencies`.
4. Run `cargo spatial download sdk --sdk-version 14.0.0` to download the C API dependencies.
5. Run `cd spatialos-sdk && cargo build`.

If these steps complete successfully, the `spatialos-sdk` crate has been built and linked successfully and can be used in user code.

## Running the Example Project

To run the example project, you will need to:

1. Navigate to `project-example`
2. Run `cargo spatial local launch`

This will start a local deployment of SpatialOS with one entity. The entity will have the `Example`
component described in `project-example/schema/example.schema`.
SpatialOS will automatically launch an instance of the worker you just built, which you can verify
by opening the inspector (navigate to http://localhost:21000/inspector in your web browser). If you
want to manually launch another instance of the worker, run the following command from the
`project-example` directory:

```
cargo run -- --worker-id RustWorker999 --worker-type RustWorker receptionist
```

This will allow you to see the log output of the worker as it runs.

## Running the test-suite

To build & run the test suite you will need to:

1. `cd test-suite`
2. `mkdir schema && cargo spatial codegen`
3. `cargo test`

## Testing the code generator

To regenerate the schema bundle, run the following:

```
./dependencies/schema-compiler/schema_compiler --schema_path=project-example/schema --schema_path=dependencies/std-lib project-example/schema/example.schema --bundle_json_out=spatialos-sdk-code-generator/data/test.sb.json
```

To run the code generator tests, run the following:

```
cargo test -p spatialos-sdk-code-generator
```

To display Rusts autogenerated debug representation of the schema bundle, run the following:

```
cargo test -p spatialos-sdk-code-generator -- --nocapture
```

### Updating Rust bindings

To update the Rust bindings found in `spatialos-sdk-sys` run the following command from the root of the repository:

```bash
cargo run --bin generate_bindings -- -i ./dependencies/headers/include/improbable/ -o ./spatialos-sdk-sys/src/
```

Note that this depends on `bindgen` which has `clang` as a dependency. See [here](https://rust-lang.github.io/rust-bindgen/requirements.html) for more info.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

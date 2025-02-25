# TonStruct

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/supadupadao/tonstruct/ci.yml?label=CI)](https://github.com/supadupadao/tonstruct/actions)
[![GitHub License](https://img.shields.io/github/license/supadupadao/tonstruct)](https://github.com/supadupadao/tonstruct/blob/master/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/tonstruct)](https://crates.io/crates/tonstruct)
[![Codecov](https://img.shields.io/codecov/c/github/supadupadao/tonstruct)](https://app.codecov.io/gh/supadupadao/tonstruct)
[![Work in progress](https://img.shields.io/badge/WORK%20IN%20PROGRESS-DO%20NOT%20USE%20IN%20PRODUCTION-ff0000)](https://github.com/supadupadao/tonstruct/issues)

ℹ️ The Open Network *de*serialization crate for Rust language.

❤️ Any contributions are welcome. Feel free to open pull requests, issues, bug reports, feature proposals or anything
else

## Usage

First you need to add `tonstruct` to your project

```toml
tonstruct = { version = "*" }
```

Then add `FromCell` or `ToCell` derive macro to structures you want to serialize/deserialize

More info about how to use this crate in your project you can find in the following documents:

- [docs.rs](https://docs.rs/tonstruct/latest/tonstruct/)
- [examples](examples) directory:
    - [serialization](examples/serialize.rs) example
    - [deserialization](examples/deserialize.rs) example
    - [custom structure](examples/custom_type.rs) implementation example

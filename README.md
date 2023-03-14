# Astria Executor

Provides an RPC server for Astria packages to interface with the EVM.

### Protos and Buf Build

[Buf Build](https://buf.build/) is a platform and registry for sharing Protocol Buffers between team members. It also comes with a set of tools to generate gRPC servers and clients in a range of languages.

[Astria's Buf Build organization](https://buf.build/astria)

First, install Buf CLI and authenticate yourself:

* `$ brew install bufbuild/buf/buf` - using homebrew
    * [other ways to install](https://docs.buf.build/installation)

### Running for development

* run `buf generate buf.build/astria/execution-apis` to generate necessary types and server classes from the remote buf registry

* run `cargo run`

# Megazord sample project

In the
[Mozilla application-services](https://github.com/mozilla/application-services)
ecosystem, a "megazord" is slang for a Rust library that encapsulates
two or more standalone Rust libraries.  That is, a megazord library
exposes features from two independent libraries but by virtue of being
built and linked once, can ship only a single copy of shared
functionality.  Since the Rust standard library is large, as are
foundational Rust crates like `log`, sharing the binary code is
important.  And in the case of `log`, having multiple implementations
to configure is a frustration for consumers.

This sample project provides a playground in which to quickly
experiment with megazord options.  This is follow-up to initial
experiments in https://github.com/mozilla/application-services/pull/168.

## Structure

This playground has a top-level "megazord" library (which itself
exposes a `megazord_fn` to external consumers) that encapsulates two
libraries "foo" and "bar" (which expose a `foo_fn` and a `bar_fn`,
respectively, to external consumers).  The trick is to arrange for all
three symbols to be exposed in the final `libmegazord.so` on Android.

Be aware that transitive exposing functions to external consumers
appears to be under-specified in Rust.

## Development

Build with invocations like

```shell
env RUSTFLAGS="-C linker=/Users/nalexander/.mozbuild/android-ndk-toolchain/x86-21/bin/i686-linux-android-clang" cargo build --target i686-linux-android
```

Observe the exposed symbols with invocations like

```shell
~/.mozbuild/android-ndk-r15c/toolchains/x86_64-4.9/prebuilt/darwin-x86_64/bin/x86_64-linux-android-nm target/i686-linux-android//debug/libmegazord.so | grep fn
```
